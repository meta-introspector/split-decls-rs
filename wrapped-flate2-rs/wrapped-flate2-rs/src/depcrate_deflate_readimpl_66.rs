// Generated macro for impl_66 (impl)
macro_rules! Depcrate_deflate_readimpl_66 {
() => {
// Module: crate::deflate::read
// Provides: {"impl_66"}
// Dependencies: {}
impl < R : Read > Read for DeflateDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . inner . read (into) } }
};
}
