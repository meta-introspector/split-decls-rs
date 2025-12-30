// Generated macro for load16_aligned (function)
macro_rules! Depcrate_simd_funcsload16_aligned {
() => {
// Module: crate::simd_funcs
// Provides: {"load16_aligned"}
// Dependencies: {}
# [doc = " Safety invariant: ptr must be valid for an aligned-for-u8x16 read of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn load16_aligned (ptr : * const u8) -> u8x16 { * (ptr as * const u8x16) }
};
}
