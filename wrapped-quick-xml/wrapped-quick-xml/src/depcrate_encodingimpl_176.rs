// Generated macro for impl_176 (impl)
macro_rules! Depcrate_encodingimpl_176 {
() => {
// Module: crate::encoding
// Provides: {"impl_176"}
// Dependencies: {}
impl Decoder { pub (crate) const fn utf8 () -> Self { Decoder { # [cfg (feature = "encoding")] encoding : UTF_8 , } } # [cfg (all (test , feature = "encoding" , feature = "serialize"))] pub (crate) const fn utf16 () -> Self { Decoder { encoding : UTF_16LE } } }
};
}
