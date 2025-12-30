// Generated macro for impl_76 (impl)
macro_rules! Depcrate_deflate_writeimpl_76 {
() => {
// Module: crate::deflate::write
// Provides: {"impl_76"}
// Dependencies: {}
impl < W : Read + Write > Read for DeflateEncoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . get_mut () . read (buf) } }
};
}
