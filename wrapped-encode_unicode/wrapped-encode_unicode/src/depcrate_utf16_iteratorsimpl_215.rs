// Generated macro for impl_215 (impl)
macro_rules! Depcrate_utf16_iteratorsimpl_215 {
() => {
// Module: crate::utf16_iterators
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'a > Utf16CharIndices < 'a > { # [doc = " Extract the remainder of the source `str`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use encode_unicode::{StrExt, Utf16Char};"] # [doc = " let mut iter = \"abc\".utf16char_indices();"] # [doc = " assert_eq!(iter.next_back(), Some((2, Utf16Char::from('c'))));"] # [doc = " assert_eq!(iter.next(), Some((0, Utf16Char::from('a'))));"] # [doc = " assert_eq!(iter.as_str(), \"b\");"] # [doc = " ```"] pub fn as_str (& self) -> & 'a str { & self . str [self . index ..] } }
};
}
