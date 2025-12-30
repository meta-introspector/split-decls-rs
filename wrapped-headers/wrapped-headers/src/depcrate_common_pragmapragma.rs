// Generated macro for Pragma (struct)
macro_rules! Depcrate_common_pragmaPragma {
() => {
// Module: crate::common::pragma
// Provides: {"Pragma"}
// Dependencies: {}
# [doc = " The `Pragma` header defined by HTTP/1.0."] # [doc = ""] # [doc = " > The \"Pragma\" header field allows backwards compatibility with"] # [doc = " > HTTP/1.0 caches, so that clients can specify a \"no-cache\" request"] # [doc = " > that they will understand (as Cache-Control was not defined until"] # [doc = " > HTTP/1.1).  When the Cache-Control header field is also present and"] # [doc = " > understood in a request, Pragma is ignored."] # [doc = " > In HTTP/1.0, Pragma was defined as an extensible field for"] # [doc = " > implementation-specified directives for recipients.  This"] # [doc = " > specification deprecates such extensions to improve interoperability."] # [doc = ""] # [doc = " Spec: [https://tools.ietf.org/html/rfc7234#section-5.4][url]"] # [doc = ""] # [doc = " [url]: https://tools.ietf.org/html/rfc7234#section-5.4"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Pragma;"] # [doc = ""] # [doc = " let pragma = Pragma::no_cache();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct Pragma (HeaderValue) ;
};
}
