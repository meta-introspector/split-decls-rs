// Generated macro for ContentRange (struct)
macro_rules! Depcrate_common_content_rangeContentRange {
() => {
// Module: crate::common::content_range
// Provides: {"ContentRange"}
// Dependencies: {}
# [doc = " Content-Range, described in [RFC7233](https://tools.ietf.org/html/rfc7233#section-4.2)"] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Content-Range       = byte-content-range"] # [doc = "                     / other-content-range"] # [doc = ""] # [doc = " byte-content-range  = bytes-unit SP"] # [doc = "                       ( byte-range-resp / unsatisfied-range )"] # [doc = ""] # [doc = " byte-range-resp     = byte-range \"/\" ( complete-length / \"*\" )"] # [doc = " byte-range          = first-byte-pos \"-\" last-byte-pos"] # [doc = " unsatisfied-range   = \"*/\" complete-length"] # [doc = ""] # [doc = " complete-length     = 1*DIGIT"] # [doc = ""] # [doc = " other-content-range = other-range-unit SP other-range-resp"] # [doc = " other-range-resp    = *CHAR"] # [doc = " ```"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::ContentRange;"] # [doc = ""] # [doc = " // 100 bytes (included byte 199), with a full length of 3,400"] # [doc = " let cr = ContentRange::bytes(100..200, 3400).unwrap();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct ContentRange { # [doc = " First and last bytes of the range, omitted if request could not be"] # [doc = " satisfied"] range : Option < (u64 , u64) > , # [doc = " Total length of the instance, can be omitted if unknown"] complete_length : Option < u64 > , }
};
}
