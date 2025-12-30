// Generated macro for macro_48 (macro)
macro_rules! Depcrate_simd_funcsmacro_48 {
() => {
// Module: crate::simd_funcs
// Provides: {"macro_48"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_feature = "sse2")] { # [inline (always)] pub fn mask_ascii (s : u8x16) -> i32 { unsafe { _mm_movemask_epi8 (s . into ()) } } } else { } }
};
}
