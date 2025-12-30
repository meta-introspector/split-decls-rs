// Generated macro for impl_62 (impl)
macro_rules! Depcrate_readimpl_62 {
() => {
// Module: crate::read
// Provides: {"impl_62"}
// Dependencies: {}
impl < R : Read > Read for XzEncoder < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
};
}
