// Generated macro for impl_338 (impl)
macro_rules! Depcrate_zlib_readimpl_338 {
() => {
// Module: crate::zlib::read
// Provides: {"impl_338"}
// Dependencies: {}
impl < R : Read + Write > Write for ZlibDecoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
