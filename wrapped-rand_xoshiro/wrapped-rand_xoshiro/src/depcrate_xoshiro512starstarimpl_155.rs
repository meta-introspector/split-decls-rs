// Generated macro for impl_155 (impl)
macro_rules! Depcrate_xoshiro512starstarimpl_155 {
() => {
// Module: crate::xoshiro512starstar
// Provides: {"impl_155"}
// Dependencies: {}
impl RngCore for Xoshiro512StarStar { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let result_starstar = starstar_u64 ! (self . s [1]) ; impl_xoshiro_large ! (self) ; result_starstar } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
