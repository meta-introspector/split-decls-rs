// Generated macro for impl_107 (impl)
macro_rules! Depcrate_xoshiro256plusimpl_107 {
() => {
// Module: crate::xoshiro256plus
// Provides: {"impl_107"}
// Dependencies: {}
impl RngCore for Xoshiro256Plus { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let result_plus = self . s [0] . wrapping_add (self . s [3]) ; impl_xoshiro_u64 ! (self) ; result_plus } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
