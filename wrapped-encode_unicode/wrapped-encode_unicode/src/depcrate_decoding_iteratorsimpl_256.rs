// Generated macro for impl_256 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_256 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_256"}
// Dependencies: {}
impl < 'a > Utf16CharDecoder < 'a > { # [doc = " Extract the remainder of the source slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Unlike `Utf16CharMerger::into_inner()`, the unit after an error is never swallowed:"] # [doc = " ```"] # [doc = " # use encode_unicode::SliceExt;"] # [doc = " # use encode_unicode::error::Utf16PairError;"] # [doc = " let mut iter = [0xd901, 'F' as u16, 'S' as u16].utf16char_indices();"] # [doc = " assert_eq!(iter.next(), Some((0, Err(Utf16PairError::UnmatchedLeadingSurrogate), 1)));"] # [doc = " assert_eq!(iter.as_slice(), &['F' as u16, 'S' as u16]);"] # [doc = " ```"] pub fn as_slice (& self) -> & [u16] { & self . slice [self . index ..] } }
};
}
