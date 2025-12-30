// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl Hasher for Adler32 { # [inline] fn finish (& self) -> u64 { u64 :: from (self . checksum ()) } fn write (& mut self , bytes : & [u8]) { self . write_slice (bytes) ; } }
};
}
