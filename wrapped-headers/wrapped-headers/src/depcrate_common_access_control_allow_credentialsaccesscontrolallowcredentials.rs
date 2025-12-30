// Generated macro for AccessControlAllowCredentials (struct)
macro_rules! Depcrate_common_access_control_allow_credentialsAccessControlAllowCredentials {
() => {
// Module: crate::common::access_control_allow_credentials
// Provides: {"AccessControlAllowCredentials"}
// Dependencies: {}
# [doc = " `Access-Control-Allow-Credentials` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)"] # [doc = ""] # [doc = " > The Access-Control-Allow-Credentials HTTP response header indicates whether the"] # [doc = " > response to request can be exposed when the credentials flag is true. When part"] # [doc = " > of the response to an preflight request it indicates that the actual request can"] # [doc = " > be made with credentials. The Access-Control-Allow-Credentials HTTP header must"] # [doc = " > match the following ABNF:"] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Allow-Credentials: \"Access-Control-Allow-Credentials\" \":\" \"true\""] # [doc = " ```"] # [doc = ""] # [doc = " Since there is only one acceptable field value, the header struct does not accept"] # [doc = " any values at all. Setting an empty `AccessControlAllowCredentials` header is"] # [doc = " sufficient. See the examples below."] # [doc = ""] # [doc = " # Example values"] # [doc = " * \"true\""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::AccessControlAllowCredentials;"] # [doc = ""] # [doc = " let allow_creds = AccessControlAllowCredentials;"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq , Debug)] pub struct AccessControlAllowCredentials ;
};
}
