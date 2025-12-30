// Generated macro for macro_49 (macro)
macro_rules! Depcrate_simd_funcsmacro_49 {
() => {
// Module: crate::simd_funcs
// Provides: {"macro_49"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_feature = "sse2")] { # [inline (always)] pub fn simd_is_ascii (s : u8x16) -> bool { unsafe { _mm_movemask_epi8 (s . into ()) == 0 } } } else if # [cfg (target_arch = "aarch64")] { # [inline (always)] pub fn simd_is_ascii (s : u8x16) -> bool { unsafe { vmaxvq_u8 (s . into ()) < 0x80 } } } else { # [inline (always)] pub fn simd_is_ascii (s : u8x16) -> bool { let highest_ascii = u8x16 :: splat (0x7F) ; ! any_mask8x16 (s . simd_gt (highest_ascii)) } } }
};
}
