// Generated macro for impl_36 (impl)
macro_rules! Depcrate_xoroshiro128plusimpl_36 {
() => {
// Module: crate::xoroshiro128plus
// Provides: {"impl_36"}
// Dependencies: {}
impl RngCore for Xoroshiro128Plus { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let r = self . s0 . wrapping_add (self . s1) ; impl_xoroshiro_u64 ! (self) ; r } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
