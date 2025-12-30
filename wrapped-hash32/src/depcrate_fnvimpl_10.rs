// Generated macro for impl_10 (impl)
macro_rules! Depcrate_fnvimpl_10 {
() => {
// Module: crate::fnv
// Provides: {"impl_10"}
// Dependencies: {}
impl core :: hash :: Hasher for FnvHasher { # [inline] fn write (& mut self , bytes : & [u8]) { for byte in bytes { self . state ^= u32 :: from (* byte) ; self . state = self . state . wrapping_mul (PRIME) ; } } # [inline] fn finish (& self) -> u64 { self . finish32 () . into () } }
};
}
