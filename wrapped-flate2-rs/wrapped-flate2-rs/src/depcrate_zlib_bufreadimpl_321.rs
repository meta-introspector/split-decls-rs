// Generated macro for impl_321 (impl)
macro_rules! Depcrate_zlib_bufreadimpl_321 {
() => {
// Module: crate::zlib::bufread
// Provides: {"impl_321"}
// Dependencies: {}
impl < R : BufRead + Write > Write for ZlibDecoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
