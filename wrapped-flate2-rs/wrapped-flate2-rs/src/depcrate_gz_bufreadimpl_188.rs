// Generated macro for impl_188 (impl)
macro_rules! Depcrate_gz_bufreadimpl_188 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_188"}
// Dependencies: {}
impl < R : BufRead + Write > Write for GzDecoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
