// Generated macro for impl_314 (impl)
macro_rules! Depcrate_zlib_bufreadimpl_314 {
() => {
// Module: crate::zlib::bufread
// Provides: {"impl_314"}
// Dependencies: {}
impl < R : BufRead > Read for ZlibEncoder < R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { zio :: read (& mut self . obj , & mut self . data , buf) } }
};
}
