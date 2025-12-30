// Generated macro for AccessControlAllowHeaders (struct)
macro_rules! Depcrate_common_access_control_allow_headersAccessControlAllowHeaders {
() => {
// Module: crate::common::access_control_allow_headers
// Provides: {"AccessControlAllowHeaders"}
// Dependencies: {}
# [doc = " `Access-Control-Allow-Headers` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)"] # [doc = ""] # [doc = " The `Access-Control-Allow-Headers` header indicates, as part of the"] # [doc = " response to a preflight request, which header field names can be used"] # [doc = " during the actual request."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Allow-Headers: \"Access-Control-Allow-Headers\" \":\" #field-name"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `accept-language, date`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = " use http::header::{CACHE_CONTROL, CONTENT_TYPE};"] # [doc = " use headers::AccessControlAllowHeaders;"] # [doc = ""] # [doc = " let allow_headers = vec![CACHE_CONTROL, CONTENT_TYPE]"] # [doc = "     .into_iter()"] # [doc = "     .collect::<AccessControlAllowHeaders>();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct AccessControlAllowHeaders (FlatCsv) ;
};
}
