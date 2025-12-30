// Generated macro for simd_byte_swap (function)
macro_rules! Depcrate_simd_funcssimd_byte_swap {
() => {
// Module: crate::simd_funcs
// Provides: {"simd_byte_swap"}
// Dependencies: {}
# [inline (always)] pub fn simd_byte_swap (s : u16x8) -> u16x8 { let left = s << 8 ; let right = s >> 8 ; left | right }
};
}
