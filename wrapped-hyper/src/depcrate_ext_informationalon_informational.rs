// Generated macro for on_informational (function)
macro_rules! Depcrate_ext_informationalon_informational {
() => {
// Module: crate::ext::informational
// Provides: {"on_informational"}
// Dependencies: {}
# [doc = " Add a callback for 1xx informational responses."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # let some_body = ();"] # [doc = " let mut req = hyper::Request::new(some_body);"] # [doc = ""] # [doc = " hyper::ext::on_informational(&mut req, |res| {"] # [doc = "     println!(\"informational: {:?}\", res.status());"] # [doc = " });"] # [doc = ""] # [doc = " // send request on a client connection..."] # [doc = " ```"] pub fn on_informational < B , F > (req : & mut http :: Request < B > , callback : F) where F : Fn (Response < '_ >) + Send + Sync + 'static , { on_informational_raw (req , OnInformationalClosure (callback)) ; }
};
}
