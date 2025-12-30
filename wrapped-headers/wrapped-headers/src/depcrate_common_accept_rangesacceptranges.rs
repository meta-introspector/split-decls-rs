// Generated macro for AcceptRanges (struct)
macro_rules! Depcrate_common_accept_rangesAcceptRanges {
() => {
// Module: crate::common::accept_ranges
// Provides: {"AcceptRanges"}
// Dependencies: {}
# [doc = " `Accept-Ranges` header, defined in [RFC7233](https://datatracker.ietf.org/doc/html/rfc7233#section-2.3)"] # [doc = ""] # [doc = " The `Accept-Ranges` header field allows a server to indicate that it"] # [doc = " supports range requests for the target resource."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Accept-Ranges     = acceptable-ranges"] # [doc = " acceptable-ranges = 1#range-unit / \\\"none\\\""] # [doc = ""] # [doc = " # Example values"] # [doc = " * `bytes`"] # [doc = " * `none`"] # [doc = " * `unknown-unit`"] # [doc = " ```"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::{AcceptRanges, HeaderMap, HeaderMapExt};"] # [doc = ""] # [doc = " let mut headers = HeaderMap::new();"] # [doc = ""] # [doc = " headers.typed_insert(AcceptRanges::bytes());"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct AcceptRanges (FlatCsv) ;
};
}
