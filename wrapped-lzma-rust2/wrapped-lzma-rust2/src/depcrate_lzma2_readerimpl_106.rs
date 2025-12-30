// Generated macro for impl_106 (impl)
macro_rules! Depcrate_lzma2_readerimpl_106 {
() => {
// Module: crate::lzma2_reader
// Provides: {"impl_106"}
// Dependencies: {}
impl < R > Lzma2Reader < R > { # [doc = " Unwraps the reader, returning the underlying reader."] pub fn into_inner (self) -> R { self . inner } # [doc = " Returns a reference to the inner reader."] pub fn inner (& self) -> & R { & self . inner } # [doc = " Returns a mutable reference to the inner reader."] pub fn inner_mut (& mut self) -> & mut R { & mut self . inner } }
};
}
