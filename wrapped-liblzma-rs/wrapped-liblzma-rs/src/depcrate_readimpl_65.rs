// Generated macro for impl_65 (impl)
macro_rules! Depcrate_readimpl_65 {
() => {
// Module: crate::read
// Provides: {"impl_65"}
// Dependencies: {}
impl < R : Read > Read for XzDecoder < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
