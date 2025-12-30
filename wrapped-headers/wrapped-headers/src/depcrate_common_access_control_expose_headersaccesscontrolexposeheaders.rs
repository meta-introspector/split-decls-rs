// Generated macro for AccessControlExposeHeaders (struct)
macro_rules! Depcrate_common_access_control_expose_headersAccessControlExposeHeaders {
() => {
// Module: crate::common::access_control_expose_headers
// Provides: {"AccessControlExposeHeaders"}
// Dependencies: {}
# [doc = " `Access-Control-Expose-Headers` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-expose-headers-response-header)"] # [doc = ""] # [doc = " The Access-Control-Expose-Headers header indicates which headers are safe to expose to the"] # [doc = " API of a CORS API specification."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Expose-Headers = \"Access-Control-Expose-Headers\" \":\" #field-name"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `ETag, Content-Length`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = " # fn main() {"] # [doc = " use http::header::{CONTENT_LENGTH, ETAG};"] # [doc = " use headers::AccessControlExposeHeaders;"] # [doc = ""] # [doc = " let expose = vec![CONTENT_LENGTH, ETAG]"] # [doc = "     .into_iter()"] # [doc = "     .collect::<AccessControlExposeHeaders>();"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct AccessControlExposeHeaders (FlatCsv) ;
};
}
