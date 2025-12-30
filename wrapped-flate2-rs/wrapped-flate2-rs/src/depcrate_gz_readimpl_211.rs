// Generated macro for impl_211 (impl)
macro_rules! Depcrate_gz_readimpl_211 {
() => {
// Module: crate::gz::read
// Provides: {"impl_211"}
// Dependencies: {}
impl < R : Read + Write > Write for GzDecoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
