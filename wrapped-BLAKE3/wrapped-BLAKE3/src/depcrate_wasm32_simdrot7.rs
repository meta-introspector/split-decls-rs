// Generated macro for rot7 (function)
macro_rules! Depcrate_wasm32_simdrot7 {
() => {
// Module: crate::wasm32_simd
// Provides: {"rot7"}
// Dependencies: {}
# [inline (always)] fn rot7 (a : v128) -> v128 { v128_or (u32x4_shr (a , 7) , u32x4_shl (a , 32 - 7)) }
};
}
