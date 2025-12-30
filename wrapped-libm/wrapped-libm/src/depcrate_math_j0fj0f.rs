// Generated macro for j0f (function)
macro_rules! Depcrate_math_j0fj0f {
() => {
// Module: crate::math::j0f
// Provides: {"j0f"}
// Dependencies: {}
# [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j0f (mut x : f32) -> f32 { let z : f32 ; let r : f32 ; let s : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 1.0 / (x * x) ; } x = fabsf (x) ; if ix >= 0x40000000 { return common (ix , x , false) ; } if ix >= 0x3a000000 { z = x * x ; r = z * (R02 + z * (R03 + z * (R04 + z * R05))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * S04))) ; return (1.0 + x / 2.0) * (1.0 - x / 2.0) + z * (r / s) ; } if ix >= 0x21800000 { x = 0.25 * x * x ; } return 1.0 - x ; }
};
}
