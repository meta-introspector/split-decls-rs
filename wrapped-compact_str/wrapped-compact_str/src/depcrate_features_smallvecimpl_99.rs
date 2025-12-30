// Generated macro for impl_99 (impl)
macro_rules! Depcrate_features_smallvecimpl_99 {
() => {
// Module: crate::features::smallvec
// Provides: {"impl_99"}
// Dependencies: {}
impl CompactString { # [doc = " Converts a [`CompactString`] into a byte vector"] # [doc = ""] # [doc = " This consumes the [`CompactString`] and returns a [`SmallVec`], so we do not need to copy"] # [doc = " contents"] # [doc = ""] # [doc = " Note: [`SmallVec`] is an inline-able version [`Vec`](alloc::vec::Vec), just like"] # [doc = " [`CompactString`] is an inline-able version of [`String`](alloc::string::String)."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use compact_str::CompactString;"] # [doc = ""] # [doc = " let c = CompactString::new(\"hello\");"] # [doc = " let bytes = c.into_bytes();"] # [doc = ""] # [doc = " assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "smallvec")))] pub fn into_bytes (self) -> SmallVec < [u8 ; MAX_SIZE] > { self . 0 . into_bytes () } }
};
}
