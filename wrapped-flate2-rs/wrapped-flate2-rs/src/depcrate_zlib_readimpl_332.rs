// Generated macro for impl_332 (impl)
macro_rules! Depcrate_zlib_readimpl_332 {
() => {
// Module: crate::zlib::read
// Provides: {"impl_332"}
// Dependencies: {}
impl < R : Read > Read for ZlibEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
