// Generated macro for macro_50 (macro)
macro_rules! Depcrate_simd_funcsmacro_50 {
() => {
// Module: crate::simd_funcs
// Provides: {"macro_50"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_feature = "sse2")] { # [inline (always)] pub fn simd_is_str_latin1 (s : u8x16) -> bool { if simd_is_ascii (s) { return true ; } let above_str_latin1 = u8x16 :: splat (0xC4) ; s . simd_lt (above_str_latin1) . all () } } else if # [cfg (target_arch = "aarch64")] { # [inline (always)] pub fn simd_is_str_latin1 (s : u8x16) -> bool { unsafe { vmaxvq_u8 (s . into ()) < 0xC4 } } } else { # [inline (always)] pub fn simd_is_str_latin1 (s : u8x16) -> bool { let above_str_latin1 = u8x16 :: splat (0xC4) ; all_mask8x16 (s . simd_lt (above_str_latin1)) } } }
};
}
