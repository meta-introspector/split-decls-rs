// Generated macro for impl_325 (impl)
macro_rules! Depcrate_responseimpl_325 {
() => {
// Module: crate::response
// Provides: {"impl_325"}
// Dependencies: {}
impl Response < () > { # [doc = " Creates a new builder-style object to manufacture a `Response`"] # [doc = ""] # [doc = " This method returns an instance of `Builder` which can be used to"] # [doc = " create a `Response`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use http::*;"] # [doc = " let response = Response::builder()"] # [doc = "     .status(200)"] # [doc = "     .header(\"X-Custom-Foo\", \"Bar\")"] # [doc = "     .body(())"] # [doc = "     .unwrap();"] # [doc = " ```"] # [inline] pub fn builder () -> Builder { Builder :: new () } }
};
}
