// Generated macro for impl_98 (impl)
macro_rules! Depcrate_xoshiro128starstarimpl_98 {
() => {
// Module: crate::xoshiro128starstar
// Provides: {"impl_98"}
// Dependencies: {}
impl RngCore for Xoshiro128StarStar { # [inline] fn next_u32 (& mut self) -> u32 { let result_starstar = starstar_u64 ! (self . s [1]) ; impl_xoshiro_u32 ! (self) ; result_starstar } # [inline] fn next_u64 (& mut self) -> u64 { next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
