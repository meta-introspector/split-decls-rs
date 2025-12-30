// Generated macro for IfUnmodifiedSince (struct)
macro_rules! Depcrate_common_if_unmodified_sinceIfUnmodifiedSince {
() => {
// Module: crate::common::if_unmodified_since
// Provides: {"IfUnmodifiedSince"}
// Dependencies: {}
# [doc = " `If-Unmodified-Since` header, defined in"] # [doc = " [RFC7232](https://datatracker.ietf.org/doc/html/rfc7232#section-3.4)"] # [doc = ""] # [doc = " The `If-Unmodified-Since` header field makes the request method"] # [doc = " conditional on the selected representation's last modification date"] # [doc = " being earlier than or equal to the date provided in the field-value."] # [doc = " This field accomplishes the same purpose as If-Match for cases where"] # [doc = " the user agent does not have an entity-tag for the representation."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " If-Unmodified-Since = HTTP-date"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `Sat, 29 Oct 1994 19:43:31 GMT`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::IfUnmodifiedSince;"] # [doc = " use std::time::{SystemTime, Duration};"] # [doc = ""] # [doc = " let time = SystemTime::now() - Duration::from_secs(60 * 60 * 24);"] # [doc = " let if_unmod = IfUnmodifiedSince::from(time);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct IfUnmodifiedSince (HttpDate) ;
};
}
