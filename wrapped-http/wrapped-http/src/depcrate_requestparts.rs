// Generated macro for Parts (struct)
macro_rules! Depcrate_requestParts {
() => {
// Module: crate::request
// Provides: {"Parts"}
// Dependencies: {}
# [doc = " Component parts of an HTTP `Request`"] # [doc = ""] # [doc = " The HTTP request head consists of a method, uri, version, and a set of"] # [doc = " header fields."] # [derive (Clone)] pub struct Parts { # [doc = " The request's method"] pub method : Method , # [doc = " The request's URI"] pub uri : Uri , # [doc = " The request's version"] pub version : Version , # [doc = " The request's headers"] pub headers : HeaderMap < HeaderValue > , # [doc = " The request's extensions"] pub extensions : Extensions , _priv : () , }
};
}
