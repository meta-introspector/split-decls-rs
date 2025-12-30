// Generated macro for impl_10 (impl)
macro_rules! Depcrate_fnvimpl_10 {
() => {
// Module: crate::fnv
// Provides: {"impl_10"}
// Dependencies: {}
impl std :: hash :: Hasher for Hasher { fn finish (& self) -> u64 { self . 0 } fn write (& mut self , bytes : & [u8]) { for & byte in bytes . iter () { self . 0 = self . 0 ^ u64 :: from (byte) ; self . 0 = self . 0 . wrapping_mul (Hasher :: PRIME) ; } } }
};
}
