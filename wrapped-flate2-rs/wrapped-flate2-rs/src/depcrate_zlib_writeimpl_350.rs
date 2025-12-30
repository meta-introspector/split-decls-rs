// Generated macro for impl_350 (impl)
macro_rules! Depcrate_zlib_writeimpl_350 {
() => {
// Module: crate::zlib::write
// Provides: {"impl_350"}
// Dependencies: {}
impl < W : Write > Write for ZlibDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
