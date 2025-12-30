// Generated macro for impl_125 (impl)
macro_rules! Depcrate_xoshiro256starstarimpl_125 {
() => {
// Module: crate::xoshiro256starstar
// Provides: {"impl_125"}
// Dependencies: {}
impl RngCore for Xoshiro256StarStar { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let result_starstar = starstar_u64 ! (self . s [1]) ; impl_xoshiro_u64 ! (self) ; result_starstar } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
