// Generated macro for impl_333 (impl)
macro_rules! Depcrate_zlib_readimpl_333 {
() => {
// Module: crate::zlib::read
// Provides: {"impl_333"}
// Dependencies: {}
impl < W : Read + Write > Write for ZlibEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
