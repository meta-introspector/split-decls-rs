// Generated macro for impl_62 (impl)
macro_rules! Depcrate_deflate_readimpl_62 {
() => {
// Module: crate::deflate::read
// Provides: {"impl_62"}
// Dependencies: {}
impl < W : Read + Write > Write for DeflateEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . get_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . get_mut () . flush () } }
};
}
