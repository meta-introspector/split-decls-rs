// Generated macro for impl_80 (impl)
macro_rules! Depcrate_xoshiro128plusimpl_80 {
() => {
// Module: crate::xoshiro128plus
// Provides: {"impl_80"}
// Dependencies: {}
impl RngCore for Xoshiro128Plus { # [inline] fn next_u32 (& mut self) -> u32 { let result_plus = self . s [0] . wrapping_add (self . s [3]) ; impl_xoshiro_u32 ! (self) ; result_plus } # [inline] fn next_u64 (& mut self) -> u64 { next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
