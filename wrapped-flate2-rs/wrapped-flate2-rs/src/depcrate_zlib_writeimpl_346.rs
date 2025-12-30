// Generated macro for impl_346 (impl)
macro_rules! Depcrate_zlib_writeimpl_346 {
() => {
// Module: crate::zlib::write
// Provides: {"impl_346"}
// Dependencies: {}
impl < W : Write > Write for ZlibEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
