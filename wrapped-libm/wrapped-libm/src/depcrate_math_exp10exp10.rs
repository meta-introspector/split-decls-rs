// Generated macro for exp10 (function)
macro_rules! Depcrate_math_exp10exp10 {
() => {
// Module: crate::math::exp10
// Provides: {"exp10"}
// Dependencies: {}
# [doc = " Calculates 10 raised to the power of `x` (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn exp10 (x : f64) -> f64 { let (mut y , n) = modf (x) ; let u : u64 = n . to_bits () ; if ((u >> 52) & 0x7ff) < 0x3ff + 4 { if y == 0.0 { return i ! (P10 , ((n as isize) + 15) as usize) ; } y = exp2 (LN10 * y) ; return y * i ! (P10 , ((n as isize) + 15) as usize) ; } return pow (10.0 , x) ; }
};
}
