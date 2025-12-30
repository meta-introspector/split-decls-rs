// Generated macro for impl_245 (impl)
macro_rules! Depcrate_decoding_iteratorsimpl_245 {
() => {
// Module: crate::decoding_iterators
// Provides: {"impl_245"}
// Dependencies: {}
impl < 'a > Utf8CharDecoder < 'a > { # [doc = " Extract the remainder of the source slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Unlike `Utf8CharMerger::into_inner()`, bytes directly after an error"] # [doc = " are never swallowed:"] # [doc = " ```"] # [doc = " # use encode_unicode::SliceExt;"] # [doc = " let mut iter = b\"\\xf4\\xa1\\xb2FS\".utf8char_indices();"] # [doc = " assert!(iter.next().unwrap().1.is_err());"] # [doc = " assert_eq!(iter.as_slice(), b\"\\xa1\\xb2FS\");"] # [doc = " ```"] pub fn as_slice (& self) -> & 'a [u8] { & self . slice [self . index ..] } }
};
}
