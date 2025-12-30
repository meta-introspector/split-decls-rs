// Generated macro for impl_79 (impl)
macro_rules! Depcrate_writeimpl_79 {
() => {
// Module: crate::write
// Provides: {"impl_79"}
// Dependencies: {}
impl < W : Read + Write > Read for XzEncoder < W > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
};
}
