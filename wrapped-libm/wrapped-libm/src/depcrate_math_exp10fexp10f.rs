// Generated macro for exp10f (function)
macro_rules! Depcrate_math_exp10fexp10f {
() => {
// Module: crate::math::exp10f
// Provides: {"exp10f"}
// Dependencies: {}
# [doc = " Calculates 10 raised to the power of `x` (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn exp10f (x : f32) -> f32 { let (mut y , n) = modff (x) ; let u = n . to_bits () ; if ((u >> 23) & 0xff) < 0x7f + 3 { if y == 0.0 { return i ! (P10 , ((n as isize) + 7) as usize) ; } y = exp2f (LN10_F32 * y) ; return y * i ! (P10 , ((n as isize) + 7) as usize) ; } return exp2 (LN10_F64 * (x as f64)) as f32 ; }
};
}
