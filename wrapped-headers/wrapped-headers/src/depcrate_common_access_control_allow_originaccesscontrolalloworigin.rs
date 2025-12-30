// Generated macro for AccessControlAllowOrigin (struct)
macro_rules! Depcrate_common_access_control_allow_originAccessControlAllowOrigin {
() => {
// Module: crate::common::access_control_allow_origin
// Provides: {"AccessControlAllowOrigin"}
// Dependencies: {}
# [doc = " The `Access-Control-Allow-Origin` response header,"] # [doc = " part of [CORS](http://www.w3.org/TR/cors/#access-control-allow-origin-response-header)"] # [doc = ""] # [doc = " The `Access-Control-Allow-Origin` header indicates whether a resource"] # [doc = " can be shared based by returning the value of the Origin request header,"] # [doc = " `*`, or `null` in the response."] # [doc = ""] # [doc = " ## ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Allow-Origin = \"Access-Control-Allow-Origin\" \":\" origin-list-or-null | \"*\""] # [doc = " ```"] # [doc = ""] # [doc = " ## Example values"] # [doc = " * `null`"] # [doc = " * `*`"] # [doc = " * `http://google.com/`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::AccessControlAllowOrigin;"] # [doc = " use std::convert::TryFrom;"] # [doc = ""] # [doc = " let any_origin = AccessControlAllowOrigin::ANY;"] # [doc = " let null_origin = AccessControlAllowOrigin::NULL;"] # [doc = " let origin = AccessControlAllowOrigin::try_from(\"http://web-platform.test:8000\");"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct AccessControlAllowOrigin (OriginOrAny) ;
};
}
