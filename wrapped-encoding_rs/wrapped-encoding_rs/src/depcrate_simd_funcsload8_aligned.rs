// Generated macro for load8_aligned (function)
macro_rules! Depcrate_simd_funcsload8_aligned {
() => {
// Module: crate::simd_funcs
// Provides: {"load8_aligned"}
// Dependencies: {}
# [doc = " Safety invariant: ptr must be valid for an aligned-for-u16x8 read of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn load8_aligned (ptr : * const u16) -> u16x8 { * (ptr as * const u16x8) }
};
}
