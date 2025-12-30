// Generated macro for macro_45 (macro)
macro_rules! Depcrate_simd_funcsmacro_45 {
() => {
// Module: crate::simd_funcs
// Provides: {"macro_45"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (target_feature = "sse2" , target_arch = "x86_64"))] { use core :: arch :: x86_64 :: __m128i ; use core :: arch :: x86_64 :: _mm_movemask_epi8 ; use core :: arch :: x86_64 :: _mm_packus_epi16 ; } else if # [cfg (all (target_feature = "sse2" , target_arch = "x86"))] { use core :: arch :: x86 :: __m128i ; use core :: arch :: x86 :: _mm_movemask_epi8 ; use core :: arch :: x86 :: _mm_packus_epi16 ; } else if # [cfg (target_arch = "aarch64")] { use core :: arch :: aarch64 :: vmaxvq_u8 ; use core :: arch :: aarch64 :: vmaxvq_u16 ; } else { } }
};
}
