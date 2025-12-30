// Generated macro for impl_337 (impl)
macro_rules! Depcrate_zlib_readimpl_337 {
() => {
// Module: crate::zlib::read
// Provides: {"impl_337"}
// Dependencies: {}
impl < R : Read > Read for ZlibDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . inner . read (into) } }
};
}
