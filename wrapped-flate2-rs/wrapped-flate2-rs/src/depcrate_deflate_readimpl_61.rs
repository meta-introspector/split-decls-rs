// Generated macro for impl_61 (impl)
macro_rules! Depcrate_deflate_readimpl_61 {
() => {
// Module: crate::deflate::read
// Provides: {"impl_61"}
// Dependencies: {}
impl < R : Read > Read for DeflateEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
