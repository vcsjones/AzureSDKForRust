#![recursion_limit = "128"]
// this will force us on nightly. The alternative is to
// Box everything which is annoying. This will *eventually* become
// stable so we will support stable too.
#![feature(conservative_impl_trait)]

extern crate chrono;
#[macro_use]
extern crate hyper;
extern crate mime;

extern crate futures;
extern crate hyper_tls;
extern crate native_tls;
extern crate tokio_core;

extern crate base64;
extern crate crypto;
extern crate time;
#[macro_use]
extern crate url;
extern crate uuid;
extern crate xml;

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[macro_use]
pub mod azure;
