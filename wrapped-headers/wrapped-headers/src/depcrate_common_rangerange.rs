// Generated macro for Range (struct)
macro_rules! Depcrate_common_rangeRange {
() => {
// Module: crate::common::range
// Provides: {"Range"}
// Dependencies: {}
# [doc = " `Range` header, defined in [RFC7233](https://tools.ietf.org/html/rfc7233#section-3.1)"] # [doc = ""] # [doc = " The \"Range\" header field on a GET request modifies the method"] # [doc = " semantics to request transfer of only one or more subranges of the"] # [doc = " selected representation data, rather than the entire selected"] # [doc = " representation data."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Range = byte-ranges-specifier / other-ranges-specifier"] # [doc = " other-ranges-specifier = other-range-unit \"=\" other-range-set"] # [doc = " other-range-set = 1*VCHAR"] # [doc = ""] # [doc = " bytes-unit = \"bytes\""] # [doc = ""] # [doc = " byte-ranges-specifier = bytes-unit \"=\" byte-range-set"] # [doc = " byte-range-set = 1#(byte-range-spec / suffix-byte-range-spec)"] # [doc = " byte-range-spec = first-byte-pos \"-\" [last-byte-pos]"] # [doc = " first-byte-pos = 1*DIGIT"] # [doc = " last-byte-pos = 1*DIGIT"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `bytes=1000-`"] # [doc = " * `bytes=-2000`"] # [doc = " * `bytes=0-1,30-40`"] # [doc = " * `bytes=0-10,20-90,-100`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Range;"] # [doc = ""] # [doc = ""] # [doc = " let range = Range::bytes(0..1234).unwrap();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct Range (HeaderValue) ;
};
}
