// Generated macro for AccessControlRequestHeaders (struct)
macro_rules! Depcrate_common_access_control_request_headersAccessControlRequestHeaders {
() => {
// Module: crate::common::access_control_request_headers
// Provides: {"AccessControlRequestHeaders"}
// Dependencies: {}
# [doc = " `Access-Control-Request-Headers` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-request-headers-request-header)"] # [doc = ""] # [doc = " The `Access-Control-Request-Headers` header indicates which headers will"] # [doc = " be used in the actual request as part of the preflight request."] # [doc = " during the actual request."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Allow-Headers: \"Access-Control-Allow-Headers\" \":\" #field-name"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `accept-language, date`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = " # fn main() {"] # [doc = " use http::header::{ACCEPT_LANGUAGE, DATE};"] # [doc = " use headers::AccessControlRequestHeaders;"] # [doc = ""] # [doc = " let req_headers = vec![ACCEPT_LANGUAGE, DATE]"] # [doc = "     .into_iter()"] # [doc = "     .collect::<AccessControlRequestHeaders>();"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct AccessControlRequestHeaders (FlatCsv) ;
};
}
