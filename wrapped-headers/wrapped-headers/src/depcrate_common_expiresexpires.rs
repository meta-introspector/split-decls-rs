// Generated macro for Expires (struct)
macro_rules! Depcrate_common_expiresExpires {
() => {
// Module: crate::common::expires
// Provides: {"Expires"}
// Dependencies: {}
# [doc = " `Expires` header, defined in [RFC7234](https://datatracker.ietf.org/doc/html/rfc7234#section-5.3)"] # [doc = ""] # [doc = " The `Expires` header field gives the date/time after which the"] # [doc = " response is considered stale."] # [doc = ""] # [doc = " The presence of an Expires field does not imply that the original"] # [doc = " resource will change or cease to exist at, before, or after that"] # [doc = " time."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Expires = HTTP-date"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `Thu, 01 Dec 1994 16:00:00 GMT`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Expires;"] # [doc = " use std::time::{SystemTime, Duration};"] # [doc = ""] # [doc = " let time = SystemTime::now() + Duration::from_secs(60 * 60 * 24);"] # [doc = " let expires = Expires::from(time);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Expires (HttpDate) ;
};
}
