// Generated macro for j0 (function)
macro_rules! Depcrate_math_j0j0 {
() => {
// Module: crate::math::j0
// Provides: {"j0"}
// Dependencies: {}
# [doc = " Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j0 (mut x : f64) -> f64 { let z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x7ff00000 { return 1.0 / (x * x) ; } x = fabs (x) ; if ix >= 0x40000000 { return common (ix , x , false) ; } if ix >= 0x3f200000 { z = x * x ; r = z * (R02 + z * (R03 + z * (R04 + z * R05))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * S04))) ; return (1.0 + x / 2.0) * (1.0 - x / 2.0) + z * (r / s) ; } if ix >= 0x38000000 { x = 0.25 * x * x ; } return 1.0 - x ; }
};
}
