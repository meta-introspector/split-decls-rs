// Generated macro for AsciiSet (struct)
macro_rules! Depcrate_ascii_setAsciiSet {
() => {
// Module: crate::ascii_set
// Provides: {"AsciiSet"}
// Dependencies: {}
# [doc = " Represents a set of characters or bytes in the ASCII range."] # [doc = ""] # [doc = " This is used in [`percent_encode`] and [`utf8_percent_encode`]."] # [doc = " This is similar to [percent-encode sets](https://url.spec.whatwg.org/#percent-encoded-bytes)."] # [doc = ""] # [doc = " Use the `add` method of an existing set to define a new set. For example:"] # [doc = ""] # [doc = " [`percent_encode`]: crate::percent_encode"] # [doc = " [`utf8_percent_encode`]: crate::utf8_percent_encode"] # [doc = ""] # [doc = " ```"] # [doc = " use percent_encoding::{AsciiSet, CONTROLS};"] # [doc = ""] # [doc = " /// https://url.spec.whatwg.org/#fragment-percent-encode-set"] # [doc = " const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'\"').add(b'<').add(b'>').add(b'`');"] # [doc = " ```"] # [derive (Debug , PartialEq , Eq)] pub struct AsciiSet { mask : [Chunk ; ASCII_RANGE_LEN / BITS_PER_CHUNK] , }
};
}
