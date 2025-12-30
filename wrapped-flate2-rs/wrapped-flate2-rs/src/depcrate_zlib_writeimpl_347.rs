// Generated macro for impl_347 (impl)
macro_rules! Depcrate_zlib_writeimpl_347 {
() => {
// Module: crate::zlib::write
// Provides: {"impl_347"}
// Dependencies: {}
impl < W : Read + Write > Read for ZlibEncoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
};
}
