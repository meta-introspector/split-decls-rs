// Generated macro for ContentLocation (struct)
macro_rules! Depcrate_common_content_locationContentLocation {
() => {
// Module: crate::common::content_location
// Provides: {"ContentLocation"}
// Dependencies: {}
# [doc = " `Content-Location` header, defined in"] # [doc = " [RFC7231](https://tools.ietf.org/html/rfc7231#section-3.1.4.2)"] # [doc = ""] # [doc = " The header can be used by both the client in requests and the server"] # [doc = " in responses with different semantics. Client sets `Content-Location`"] # [doc = " to refer to the URI where original representation of the body was"] # [doc = " obtained."] # [doc = ""] # [doc = " In responses `Content-Location` represents URI for the representation"] # [doc = " that was content negotiated, created or for the response payload."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Content-Location = absolute-URI / partial-URI"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `/hypertext/Overview.html`"] # [doc = " * `http://www.example.org/hypertext/Overview.html`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [derive (Clone , Debug , PartialEq)] pub struct ContentLocation (HeaderValue) ;
};
}
