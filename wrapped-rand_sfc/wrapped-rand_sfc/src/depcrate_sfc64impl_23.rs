// Generated macro for impl_23 (impl)
macro_rules! Depcrate_sfc64impl_23 {
() => {
// Module: crate::sfc64
// Provides: {"impl_23"}
// Dependencies: {}
impl RngCore for Sfc64 { # [inline] fn next_u32 (& mut self) -> u32 { (self . next_u64 () >> 32) as u32 } # [inline] fn next_u64 (& mut self) -> u64 { let old_b = self . b ; let old_c = self . c ; let old_weyl = self . weyl ; let result = self . a . wrapping_add (old_b) . wrapping_add (old_weyl) ; self . a = old_b ^ (old_b >> RSHIFT) ; self . b = old_c . wrapping_add (old_c << LSHIFT) ; self . c = result . wrapping_add (old_c . rotate_left (BARREL_SHIFT)) ; self . weyl = self . weyl . wrapping_add (WEYL_INC) ; result } # [inline] fn fill_bytes (& mut self , dest : & mut [u8]) { fill_bytes_via_next (self , dest) ; } }
};
}
