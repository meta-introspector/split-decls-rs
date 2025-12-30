// Generated macro for contains_surrogates (function)
macro_rules! Depcrate_simd_funcscontains_surrogates {
() => {
// Module: crate::simd_funcs
// Provides: {"contains_surrogates"}
// Dependencies: {}
# [inline (always)] pub fn contains_surrogates (s : u16x8) -> bool { let mask = u16x8 :: splat (0xF800) ; let surrogate_bits = u16x8 :: splat (0xD800) ; any_mask16x8 ((s & mask) . simd_eq (surrogate_bits)) }
};
}
