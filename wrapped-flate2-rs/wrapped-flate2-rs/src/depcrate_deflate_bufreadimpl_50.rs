// Generated macro for impl_50 (impl)
macro_rules! Depcrate_deflate_bufreadimpl_50 {
() => {
// Module: crate::deflate::bufread
// Provides: {"impl_50"}
// Dependencies: {}
impl < R : BufRead > Read for DeflateDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { zio :: read (& mut self . obj , & mut self . data , into) } }
};
}
