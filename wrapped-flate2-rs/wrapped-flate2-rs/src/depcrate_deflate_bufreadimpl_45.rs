// Generated macro for impl_45 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_45 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_45"}
// Dependencies: {}
impl < W : BufRead + Write > Write for DeflateEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
