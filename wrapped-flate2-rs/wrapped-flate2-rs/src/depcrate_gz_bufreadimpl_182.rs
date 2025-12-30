// Generated macro for impl_182 (impl)
macro_rules! Depcrate_gz_bufreadimpl_182 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_182"}
// Dependencies: {}
impl < R : BufRead + Write > Write for GzEncoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
