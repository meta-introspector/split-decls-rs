// Generated macro for impl_215 (impl)
macro_rules! Depcrate_gz_readimpl_215 {
() => {
// Module: crate::gz::read
// Provides: {"impl_215"}
// Dependencies: {}
impl < R : Read > Read for MultiGzDecoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . inner . read (into) } }
};
}
