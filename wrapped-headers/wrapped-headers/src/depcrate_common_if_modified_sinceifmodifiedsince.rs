// Generated macro for IfModifiedSince (struct)
macro_rules! Depcrate_common_if_modified_sinceIfModifiedSince {
() => {
// Module: crate::common::if_modified_since
// Provides: {"IfModifiedSince"}
// Dependencies: {}
# [doc = " `If-Modified-Since` header, defined in"] # [doc = " [RFC7232](https://datatracker.ietf.org/doc/html/rfc7232#section-3.3)"] # [doc = ""] # [doc = " The `If-Modified-Since` header field makes a GET or HEAD request"] # [doc = " method conditional on the selected representation's modification date"] # [doc = " being more recent than the date provided in the field-value."] # [doc = " Transfer of the selected representation's data is avoided if that"] # [doc = " data has not changed."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " If-Modified-Since = HTTP-date"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `Sat, 29 Oct 1994 19:43:31 GMT`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::IfModifiedSince;"] # [doc = " use std::time::{Duration, SystemTime};"] # [doc = ""] # [doc = " let time = SystemTime::now() - Duration::from_secs(60 * 60 * 24);"] # [doc = " let if_mod = IfModifiedSince::from(time);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct IfModifiedSince (HttpDate) ;
};
}
