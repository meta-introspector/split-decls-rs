// Generated macro for impl_67 (impl)
macro_rules! Depcrate_deflate_readimpl_67 {
() => {
// Module: crate::deflate::read
// Provides: {"impl_67"}
// Dependencies: {}
impl < W : Read + Write > Write for DeflateDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
