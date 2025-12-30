// Generated macro for impl_80 (impl)
macro_rules! Depcrate_deflate_writeimpl_80 {
() => {
// Module: crate::deflate::write
// Provides: {"impl_80"}
// Dependencies: {}
impl < W : Read + Write > Read for DeflateDecoder < W > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . get_mut () . read (buf) } }
};
}
