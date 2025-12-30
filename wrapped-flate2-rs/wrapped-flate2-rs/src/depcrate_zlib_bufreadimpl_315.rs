// Generated macro for impl_315 (impl)
macro_rules! Depcrate_zlib_bufreadimpl_315 {
() => {
// Module: crate::zlib::bufread
// Provides: {"impl_315"}
// Dependencies: {}
impl < R : BufRead + Write > Write for ZlibEncoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
