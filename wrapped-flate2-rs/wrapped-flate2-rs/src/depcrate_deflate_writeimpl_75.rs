// Generated macro for impl_75 (impl)
macro_rules! Depcrate_deflate_writeimpl_75 {
() => {
// Module: crate::deflate::write
// Provides: {"impl_75"}
// Dependencies: {}
impl < W : Write > Write for DeflateEncoder < W > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . inner . flush () } }
};
}
