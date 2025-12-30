// Generated macro for impl_131 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_131 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a > Utf8CharIndices < 'a > { # [doc = " Extract the remainder of the source `str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use encode_unicode::{StrExt, Utf8Char};"] # [doc = " let mut iter = \"abc\".utf8char_indices();"] # [doc = " assert_eq!(iter.next_back(), Some((2, Utf8Char::from('c'))));"] # [doc = " assert_eq!(iter.next(), Some((0, Utf8Char::from('a'))));"] # [doc = " assert_eq!(iter.as_str(), \"b\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { & self . str [self . index ..] } }
};
}
