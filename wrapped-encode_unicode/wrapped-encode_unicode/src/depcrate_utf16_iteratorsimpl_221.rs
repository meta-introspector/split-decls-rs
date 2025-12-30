// Generated macro for impl_221 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_221 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > Utf16Chars < 'a > { # [doc = " Extract the remainder of the source `str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use encode_unicode::{StrExt, Utf16Char};"] # [doc = " let mut iter = \"abc\".utf16chars();"] # [doc = " assert_eq!(iter.next(), Some(Utf16Char::from('a')));"] # [doc = " assert_eq!(iter.next_back(), Some(Utf16Char::from('c')));"] # [doc = " assert_eq!(iter.as_str(), \"b\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { self . 0 . as_str () } }
};
}
