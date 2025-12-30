// Generated macro for Incoming (struct)
macro_rules! Depcrate_body_incomingIncoming {
() => {
// Module: crate::body::incoming
// Provides: {"Incoming"}
// Dependencies: {}
# [doc = " A stream of `Bytes`, used when receiving bodies from the network."] # [doc = ""] # [doc = " Note that Users should not instantiate this struct directly. When working with the hyper client,"] # [doc = " `Incoming` is returned to you in responses. Similarly, when operating with the hyper server,"] # [doc = " it is provided within requests."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " async fn echo("] # [doc = "    req: Request<hyper::body::Incoming>,"] # [doc = " ) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {"] # [doc = "    //Here, you can process `Incoming`"] # [doc = " }"] # [doc = " ```"] # [must_use = "streams do nothing unless polled"] pub struct Incoming { kind : Kind , }
};
}
