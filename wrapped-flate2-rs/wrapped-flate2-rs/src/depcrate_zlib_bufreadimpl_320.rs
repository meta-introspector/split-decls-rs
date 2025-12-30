// Generated macro for impl_320 (impl)
macro_rules! Depcrate_zlib_bufreadimpl_320 {
() => {
// Module: crate::zlib::bufread
// Provides: {"impl_320"}
// Dependencies: {}
impl < R : BufRead > Read for ZlibDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { zio :: read (& mut self . obj , & mut self . data , into) } }
};
}
