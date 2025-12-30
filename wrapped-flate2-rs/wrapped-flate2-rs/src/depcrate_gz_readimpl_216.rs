// Generated macro for impl_216 (impl)
macro_rules! Depcrate_gz_readimpl_216 {
() => {
// Module: crate::gz::read
// Provides: {"impl_216"}
// Dependencies: {}
impl < R : Read + Write > Write for MultiGzDecoder < R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
