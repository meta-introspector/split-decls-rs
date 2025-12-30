// Generated macro for impl_54 (impl)
macro_rules! Depcrate_xoroshiro128starstarimpl_54 {
() => {
// Module: crate::xoroshiro128starstar
// Provides: {"impl_54"}
// Dependencies: {}
impl RngCore for Xoroshiro128StarStar { # [inline] fn next_u32 (& mut self) -> u32 { self . next_u64 () as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let r = starstar_u64 ! (self . s0) ; impl_xoroshiro_u64 ! (self) ; r } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
