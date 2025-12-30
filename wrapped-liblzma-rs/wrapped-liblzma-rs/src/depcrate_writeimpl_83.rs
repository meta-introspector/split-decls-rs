// Generated macro for impl_83 (impl)
macro_rules! Depcrate_writeimpl_83 {
() => {
// Module: crate::write
// Provides: {"impl_83"}
// Dependencies: {}
impl < W : Read + Write > Read for XzDecoder < W > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
};
}
