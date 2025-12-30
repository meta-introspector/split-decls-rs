// Generated macro for impl_62 (impl)
macro_rules! Depcrate_xoroshiro64starimpl_62 {
() => {
// Module: crate::xoroshiro64star
// Provides: {"impl_62"}
// Dependencies: {}
impl RngCore for Xoroshiro64Star { # [inline] fn next_u32 (& mut self) -> u32 { let r = self . s0 . wrapping_mul (0x9E3779BB) ; impl_xoroshiro_u32 ! (self) ; r } # [inline] fn next_u64 (& mut self) -> u64 { next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
