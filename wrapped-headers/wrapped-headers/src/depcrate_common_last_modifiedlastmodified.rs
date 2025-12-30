// Generated macro for LastModified (struct)
macro_rules! Depcrate_common_last_modifiedLastModified {
() => {
// Module: crate::common::last_modified
// Provides: {"LastModified"}
// Dependencies: {}
# [doc = " `Last-Modified` header, defined in"] # [doc = " [RFC7232](https://datatracker.ietf.org/doc/html/rfc7232#section-2.2)"] # [doc = ""] # [doc = " The `Last-Modified` header field in a response provides a timestamp"] # [doc = " indicating the date and time at which the origin server believes the"] # [doc = " selected representation was last modified, as determined at the"] # [doc = " conclusion of handling the request."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Expires = HTTP-date"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `Sat, 29 Oct 1994 19:43:31 GMT`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::LastModified;"] # [doc = " use std::time::{Duration, SystemTime};"] # [doc = ""] # [doc = " let modified = LastModified::from("] # [doc = "     SystemTime::now() - Duration::from_secs(60 * 60 * 24)"] # [doc = " );"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct LastModified (pub (super) HttpDate) ;
};
}
