// Generated macro for j1f (function)
macro_rules! Depcrate_math_j1fj1f {
() => {
// Module: crate::math::j1f
// Provides: {"j1f"}
// Dependencies: {}
# [doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j1f (x : f32) -> f32 { let mut z : f32 ; let r : f32 ; let s : f32 ; let mut ix : u32 ; let sign : bool ; ix = x . to_bits () ; sign = (ix >> 31) != 0 ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 1.0 / (x * x) ; } if ix >= 0x40000000 { return common (ix , fabsf (x) , false , sign) ; } if ix >= 0x39000000 { z = x * x ; r = z * (R00 + z * (R01 + z * (R02 + z * R03))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * (S04 + z * S05)))) ; z = 0.5 + r / s ; } else { z = 0.5 ; } return z * x ; }
};
}
