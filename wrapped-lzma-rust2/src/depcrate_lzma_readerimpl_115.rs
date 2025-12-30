// Generated macro for impl_115 (impl)
macro_rules! Depcrate_lzma_readerimpl_115 {
() => {
// Module: crate::lzma_reader
// Provides: {"impl_115"}
// Dependencies: {}
impl < R > LzmaReader < R > { # [doc = " Unwraps the reader, returning the underlying reader."] pub fn into_inner (self) -> R { self . rc . into_inner () } # [doc = " Returns a reference to the inner reader."] pub fn inner (& self) -> & R { self . rc . inner () } # [doc = " Returns a mutable reference to the inner reader."] pub fn inner_mut (& mut self) -> & mut R { self . rc . inner_mut () } }
};
}
