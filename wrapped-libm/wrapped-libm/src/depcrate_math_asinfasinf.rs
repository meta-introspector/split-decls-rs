// Generated macro for asinf (function)
macro_rules! Depcrate_math_asinfasinf {
() => {
// Module: crate::math::asinf
// Provides: {"asinf"}
// Dependencies: {}
# [doc = " Arcsine (f32)"] # [doc = ""] # [doc = " Computes the inverse sine (arc sine) of the argument `x`."] # [doc = " Arguments to asin must be in the range -1 to 1."] # [doc = " Returns values in radians, in the range of -pi/2 to pi/2."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn asinf (mut x : f32) -> f32 { let x1p_120 = f64 :: from_bits (0x3870000000000000) ; let hx = x . to_bits () ; let ix = hx & 0x7fffffff ; if ix >= 0x3f800000 { if ix == 0x3f800000 { return ((x as f64) * PIO2 + x1p_120) as f32 ; } return 0. / (x - x) ; } if ix < 0x3f000000 { if (0x00800000 .. 0x39800000) . contains (& ix) { return x ; } return x + x * r (x * x) ; } let z = (1. - Float :: abs (x)) * 0.5 ; let s = sqrt (z as f64) ; x = (PIO2 - 2. * (s + s * (r (z) as f64))) as f32 ; if (hx >> 31) != 0 { - x } else { x } }
};
}
