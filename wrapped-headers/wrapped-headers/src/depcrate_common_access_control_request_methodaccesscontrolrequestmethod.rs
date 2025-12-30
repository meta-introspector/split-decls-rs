// Generated macro for AccessControlRequestMethod (struct)
macro_rules! Depcrate_common_access_control_request_methodAccessControlRequestMethod {
() => {
// Module: crate::common::access_control_request_method
// Provides: {"AccessControlRequestMethod"}
// Dependencies: {}
# [doc = " `Access-Control-Request-Method` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-request-method-request-header)"] # [doc = ""] # [doc = " The `Access-Control-Request-Method` header indicates which method will be"] # [doc = " used in the actual request as part of the preflight request."] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Request-Method: \\\"Access-Control-Request-Method\\\" \\\":\\\" Method"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `GET`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = " use headers::AccessControlRequestMethod;"] # [doc = " use http::Method;"] # [doc = ""] # [doc = " let req_method = AccessControlRequestMethod::from(Method::GET);"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct AccessControlRequestMethod (Method) ;
};
}
