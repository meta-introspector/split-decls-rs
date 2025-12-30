// Generated macro for impl_51 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_51 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_51"}
// Dependencies: {}
impl < W : BufRead + Write > Write for DeflateDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
