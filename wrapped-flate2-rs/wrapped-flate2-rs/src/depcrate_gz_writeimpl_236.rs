// Generated macro for impl_236 (impl)
macro_rules! Depcrate_gz_writeimpl_236 {
() => {
// Module: crate::gz::write
// Provides: {"impl_236"}
// Dependencies: {}
impl < W : Read + Write > Read for GzDecoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . get_mut () . get_mut () . read (buf) } }
};
}
