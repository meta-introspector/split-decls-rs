// Generated macro for AccessControlMaxAge (struct)
macro_rules! Depcrate_common_access_control_max_ageAccessControlMaxAge {
() => {
// Module: crate::common::access_control_max_age
// Provides: {"AccessControlMaxAge"}
// Dependencies: {}
# [doc = " `Access-Control-Max-Age` header, part of"] # [doc = " [CORS](http://www.w3.org/TR/cors/#access-control-max-age-response-header)"] # [doc = ""] # [doc = " The `Access-Control-Max-Age` header indicates how long the results of a"] # [doc = " preflight request can be cached in a preflight result cache."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Access-Control-Max-Age = \\\"Access-Control-Max-Age\\\" \\\":\\\" delta-seconds"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `531`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::time::Duration;"] # [doc = " use headers::AccessControlMaxAge;"] # [doc = ""] # [doc = " let max_age = AccessControlMaxAge::from(Duration::from_secs(531));"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct AccessControlMaxAge (Seconds) ;
};
}
