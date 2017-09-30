use std::thread;
use std::str::{self};

use hyper::{self, Client};
use tokio_core;

use metadata::{Track};
use player::Player;

use iron::{Iron};
use iron::prelude::*;
use iron::status;

#[derive(Debug, Clone)]
pub struct Httpplayer {
    url: String,
    server: String,
}

impl Httpplayer {
    pub fn new() -> Self {
        Httpplayer {
            url: String::from("http://127.0.0.1:33003?t="),
            server: String::from("localhost:33004")
        }
    }

    pub fn startServer(&self, player: &Player) -> () {
        let self_clone = self.clone();
        let player_clone = player.clone();
        thread::spawn(move || {
            println!("Server started..");
            Iron::new(move |request: &mut Request| {
                match request.url.query() {
                    Some(cmd) => {
                        println!("{:?}", cmd);
                        match cmd {
                            "play" => player_clone.play(),
                            "pause" => player_clone.pause(),
                            _ => (),
                        }
                    }
                    None => ()
                }
                println!("{:?}", request.url.query());
                Ok(Response::with((status::Ok, "success!")))
            }).http(self_clone.server).unwrap();
        });
    }

    pub fn send_status(&self, status: String, position: i64) -> () {
         let res = json!({
            "status": &status,
            "position": position,
        });

        let self_clone = self.clone();
        thread::spawn(move || {
            self_clone.send_json(res.to_string());
            println!("Update Status to {:?}", &status);
        });
    }
    pub fn send(&self, track: Track) -> () {
        let res = json!({
            "id": &track.id.to_base16(),
            "raw": &track.id.to_raw(),
        });

        let self_clone = self.clone();
        thread::spawn(move || {
            self_clone.send_json(res.to_string());
            println!("Send track {:?}", &track.name);
        });
    }

    fn send_json(&self, json: String) -> () {
        let mut paramurl = json;
        paramurl = paramurl.replace("{", "%7B");
        paramurl = paramurl.replace("}", "%7D");
        paramurl = paramurl.replace("\"", "%22");
        paramurl = paramurl.replace(":", "%3A");
        paramurl = paramurl.replace(";", "%3B");
        paramurl = paramurl.replace("[", "%5B");
        paramurl = paramurl.replace("]", "%5D");
        paramurl = paramurl.replace("=", "%3D");
        paramurl = paramurl.replace("?", "%3F");
        paramurl = paramurl.replace(",", "%2C");
        paramurl = paramurl.replace("/", "%2F");
        paramurl = paramurl.replace("#", "%23");
        paramurl = paramurl.replace("&", "%26");
        paramurl = paramurl.replace(" ", "%20");

        paramurl = self.url.clone() + &paramurl;

        let url = paramurl.parse::<hyper::Uri>().unwrap();
        if url.scheme() != Some("http") {
            println!("This example only works with 'http' URLs.");
            return;
        }

            let mut core = tokio_core::reactor::Core::new().unwrap();
            let handle = core.handle();
            let client = Client::new(&handle);

            let work = client.get(url);
            // .and_then(|res| {
            //     println!("Response: {}", res.status());

            //     res.body().for_each(|chunk| {
            //         io::stdout().write_all(&chunk).map_err(From::from)
            //     })
            // }).map(|_| {
            //     println!("\n\nDone.");
            // });

            let mut err = "Failed reach ".to_owned();
            err.push_str(paramurl.as_str());

            core.run(work)
                .ok()
                .expect(err.as_str());
    }

}