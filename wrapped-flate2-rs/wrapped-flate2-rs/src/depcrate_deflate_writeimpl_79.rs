// Generated macro for impl_79 (impl)
macro_rules! Depcrate_deflate_writeimpl_79 {
() => {
// Module: crate::deflate::write
// Provides: {"impl_79"}
// Dependencies: {}
impl < W : Write > Write for DeflateDecoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
