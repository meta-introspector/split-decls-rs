// Generated macro for impl_135 (impl)
macro_rules! Depcrate_xoshiro512plusimpl_135 {
() => {
// Module: crate::xoshiro512plus
// Provides: {"impl_135"}
// Dependencies: {}
impl RngCore for Xoshiro512Plus { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let result_plus = self . s [0] . wrapping_add (self . s [2]) ; impl_xoshiro_large ! (self) ; result_plus } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
