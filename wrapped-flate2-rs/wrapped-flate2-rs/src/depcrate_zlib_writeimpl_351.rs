// Generated macro for impl_351 (impl)
macro_rules! Depcrate_zlib_writeimpl_351 {
() => {
// Module: crate::zlib::write
// Provides: {"impl_351"}
// Dependencies: {}
impl < W : Read + Write > Read for ZlibDecoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . get_mut () . read (buf) } }
};
}
