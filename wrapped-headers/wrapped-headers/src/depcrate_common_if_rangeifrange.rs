// Generated macro for IfRange (struct)
macro_rules! Depcrate_common_if_rangeIfRange {
() => {
// Module: crate::common::if_range
// Provides: {"IfRange"}
// Dependencies: {}
# [doc = " `If-Range` header, defined in [RFC7233](https://datatracker.ietf.org/doc/html/rfc7233#section-3.2)"] # [doc = ""] # [doc = " If a client has a partial copy of a representation and wishes to have"] # [doc = " an up-to-date copy of the entire representation, it could use the"] # [doc = " Range header field with a conditional GET (using either or both of"] # [doc = " If-Unmodified-Since and If-Match.)  However, if the precondition"] # [doc = " fails because the representation has been modified, the client would"] # [doc = " then have to make a second request to obtain the entire current"] # [doc = " representation."] # [doc = ""] # [doc = " The `If-Range` header field allows a client to \\\"short-circuit\\\" the"] # [doc = " second request.  Informally, its meaning is as follows: if the"] # [doc = " representation is unchanged, send me the part(s) that I am requesting"] # [doc = " in Range; otherwise, send me the entire representation."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " If-Range = entity-tag / HTTP-date"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `Sat, 29 Oct 1994 19:43:31 GMT`"] # [doc = " * `\\\"xyzzy\\\"`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::IfRange;"] # [doc = " use std::time::{SystemTime, Duration};"] # [doc = ""] # [doc = " let fetched = SystemTime::now() - Duration::from_secs(60 * 60 * 24);"] # [doc = " let if_range = IfRange::date(fetched);"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct IfRange (IfRange_) ;
};
}
