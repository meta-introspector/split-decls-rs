// Generated macro for impl_210 (impl)
macro_rules! Depcrate_gz_readimpl_210 {
() => {
// Module: crate::gz::read
// Provides: {"impl_210"}
// Dependencies: {}
impl < R : Read > Read for GzDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . inner . read (into) } }
};
}
