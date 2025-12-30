// Generated macro for impl_117 (impl)
macro_rules! Depcrate_decodeimpl_117 {
() => {
// Module: crate::decode
// Provides: {"impl_117"}
// Dependencies: {}
impl < R : Read > Read for ReadReader < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . rd . read (buf) } # [inline] fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . rd . read_exact (buf) } }
};
}
