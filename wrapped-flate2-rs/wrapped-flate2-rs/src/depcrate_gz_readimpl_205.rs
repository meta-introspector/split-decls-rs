// Generated macro for impl_205 (impl)
macro_rules! Depcrate_gz_readimpl_205 {
() => {
// Module: crate::gz::read
// Provides: {"impl_205"}
// Dependencies: {}
impl < R : Read > Read for GzEncoder < R > { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { self . inner . read (into) } }
};
}
