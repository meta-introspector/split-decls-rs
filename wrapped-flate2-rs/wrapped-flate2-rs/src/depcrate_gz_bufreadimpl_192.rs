// Generated macro for impl_192 (impl)
macro_rules! Depcrate_gz_bufreadimpl_192 {
() => {
// Module: crate::gz::bufread
// Provides: {"impl_192"}
// Dependencies: {}
impl < R : BufRead > Read for MultiGzDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . 0 . read (into) } }
};
}
