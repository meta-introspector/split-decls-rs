// Generated macro for impl_137 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_137 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_137"}
// Dependencies: {}
impl < 'a > Utf8Chars < 'a > { # [doc = " Extract the remainder of the source `str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use encode_unicode::{StrExt, Utf8Char};"] # [doc = " let mut iter = \"abc\".utf8chars();"] # [doc = " assert_eq!(iter.next(), Some(Utf8Char::from('a')));"] # [doc = " assert_eq!(iter.next_back(), Some(Utf8Char::from('c')));"] # [doc = " assert_eq!(iter.as_str(), \"b\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { self . 0 . as_str () } }
};
}
