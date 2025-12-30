// Generated macro for impl_206 (impl)
macro_rules! Depcrate_gz_readimpl_206 {
() => {
// Module: crate::gz::read
// Provides: {"impl_206"}
// Dependencies: {}
impl < R : Read + Write > Write for GzEncoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
