// Generated macro for impl_343 (impl)
macro_rules! Depcrate_rngs_xoshiro256plusplusimpl_343 {
() => {
// Module: crate::rngs::xoshiro256plusplus
// Provides: {"impl_343"}
// Dependencies: {}
impl RngCore for Xoshiro256PlusPlus { # [inline] fn next_u32 (& mut self) -> u32 { let val = self . next_u64 () ; (val >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let res = self . s [0] . wrapping_add (self . s [3]) . rotate_left (23) . wrapping_add (self . s [0]) ; let t = self . s [1] << 17 ; self . s [2] ^= self . s [0] ; self . s [3] ^= self . s [1] ; self . s [1] ^= self . s [2] ; self . s [0] ^= self . s [3] ; self . s [2] ^= t ; self . s [3] = self . s [3] . rotate_left (45) ; res } # [inline] fn fill_bytes (& mut self , dst : & mut [u8]) { le :: fill_bytes_via_next (self , dst) } }
};
}
