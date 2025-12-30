// Generated macro for impl_44 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_44 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_44"}
// Dependencies: {}
impl < R : BufRead > Read for DeflateEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { zio :: read (& mut self . obj , & mut self . data , buf) } }
};
}
