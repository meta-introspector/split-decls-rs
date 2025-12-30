// Generated macro for AccessControlAllowMethods (struct)
macro_rules! Depcrate_common_access_control_allow_methodsAccessControlAllowMethods {
() => {
// Module: crate::common::access_control_allow_methods
// Provides: {"AccessControlAllowMethods"}
// Dependencies: {}
# [doc = " `Access-Control-Allow-Methods` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-allow-methods-response-header)"] # [doc = ""] # [doc = " The `Access-Control-Allow-Methods` header indicates, as part of the"] # [doc = " response to a preflight request, which methods can be used during the"] # [doc = " actual request."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Allow-Methods: \"Access-Control-Allow-Methods\" \":\" #Method"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `PUT, DELETE, XMODIFY`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = " use http::Method;"] # [doc = " use headers::AccessControlAllowMethods;"] # [doc = ""] # [doc = " let allow_methods = vec![Method::GET, Method::PUT]"] # [doc = "     .into_iter()"] # [doc = "     .collect::<AccessControlAllowMethods>();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct AccessControlAllowMethods (FlatCsv) ;
};
}
