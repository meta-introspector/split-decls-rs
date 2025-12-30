// Generated macro for impl_230 (impl)
macro_rules! Depcrate_gz_writeimpl_230 {
() => {
// Module: crate::gz::write
// Provides: {"impl_230"}
// Dependencies: {}
impl < R : Read + Write > Read for GzEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . get_mut () . read (buf) } }
};
}
