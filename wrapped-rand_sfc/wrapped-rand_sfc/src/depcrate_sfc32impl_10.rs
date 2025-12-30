// Generated macro for impl_10 (impl)
macro_rules! Depcrate_sfc32impl_10 {
() => {
// Module: crate::sfc32
// Provides: {"impl_10"}
// Dependencies: {}
impl RngCore for Sfc32 { # [inline] fn next_u32 (& mut self) -> u32 { let old_b = self . b ; let old_c = self . c ; let old_weyl = self . weyl ; let result = self . a . wrapping_add (old_b) . wrapping_add (old_weyl) ; self . a = old_b ^ (old_b >> RSHIFT) ; self . b = old_c . wrapping_add (old_c << LSHIFT) ; self . c = result . wrapping_add (old_c . rotate_left (BARREL_SHIFT)) ; self . weyl = old_weyl . wrapping_add (WEYL_INC) ; result } # [inline] fn next_u64 (& mut self) -> u64 { next_u64_via_u32 (self) } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
