// Generated macro for impl_70 (impl)
macro_rules! Depcrate_xoroshiro64starstarimpl_70 {
() => {
// Module: crate::xoroshiro64starstar
// Provides: {"impl_70"}
// Dependencies: {}
impl RngCore for Xoroshiro64StarStar { # [inline] fn next_u32 (& mut self) -> u32 { let r = starstar_u32 ! (self . s0) ; impl_xoroshiro_u32 ! (self) ; r } # [inline] fn next_u64 (& mut self) -> u64 { next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
