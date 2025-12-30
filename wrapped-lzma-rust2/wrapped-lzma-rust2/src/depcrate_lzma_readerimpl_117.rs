// Generated macro for impl_117 (impl)
macro_rules! Depcrate_lzma_readerimpl_117 {
() => {
// Module: crate::lzma_reader
// Provides: {"impl_117"}
// Dependencies: {}
impl < R : Read > Read for LzmaReader < R > { fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { self . read_decode (buf) } }
};
}
