// Generated macro for atan (function)
macro_rules! Depcrate_math_atanatan {
() => {
// Module: crate::math::atan
// Provides: {"atan"}
// Dependencies: {}
# [doc = " Arctangent (f64)"] # [doc = ""] # [doc = " Computes the inverse tangent (arc tangent) of the input value."] # [doc = " Returns a value in radians, in the range of -pi/2 to pi/2."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn atan (x : f64) -> f64 { let mut x = x ; let mut ix = (x . to_bits () >> 32) as u32 ; let sign = ix >> 31 ; ix &= 0x7fff_ffff ; if ix >= 0x4410_0000 { if x . is_nan () { return x ; } let z = ATANHI [3] + f64 :: from_bits (0x0380_0000) ; return if sign != 0 { - z } else { z } ; } let id = if ix < 0x3fdc_0000 { if ix < 0x3e40_0000 { if ix < 0x0010_0000 { force_eval ! (x as f32) ; } return x ; } - 1 } else { x = fabs (x) ; if ix < 0x3ff30000 { if ix < 0x3fe60000 { x = (2. * x - 1.) / (2. + x) ; 0 } else { x = (x - 1.) / (x + 1.) ; 1 } } else if ix < 0x40038000 { x = (x - 1.5) / (1. + 1.5 * x) ; 2 } else { x = - 1. / x ; 3 } } ; let z = x * x ; let w = z * z ; let s1 = z * (AT [0] + w * (AT [2] + w * (AT [4] + w * (AT [6] + w * (AT [8] + w * AT [10]))))) ; let s2 = w * (AT [1] + w * (AT [3] + w * (AT [5] + w * (AT [7] + w * AT [9])))) ; if id < 0 { return x - x * (s1 + s2) ; } let z = i ! (ATANHI , id as usize) - (x * (s1 + s2) - i ! (ATANLO , id as usize) - x) ; if sign != 0 { - z } else { z } }
};
}
