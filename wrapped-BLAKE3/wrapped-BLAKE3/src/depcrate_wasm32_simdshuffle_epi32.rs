// Generated macro for shuffle_epi32 (function)
macro_rules! Depcrate_wasm32_simdshuffle_epi32 {
() => {
// Module: crate::wasm32_simd
// Provides: {"shuffle_epi32"}
// Dependencies: {}
# [inline (always)] fn shuffle_epi32 < const I3 : usize , const I2 : usize , const I1 : usize , const I0 : usize > (a : v128 ,) -> v128 { i32x4_shuffle :: < I0 , I1 , I2 , I3 > (a , a) }
};
}
