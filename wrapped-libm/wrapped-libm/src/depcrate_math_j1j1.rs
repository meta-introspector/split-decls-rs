// Generated macro for j1 (function)
macro_rules! Depcrate_math_j1j1 {
() => {
// Module: crate::math::j1
// Provides: {"j1"}
// Dependencies: {}
# [doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn j1 (x : f64) -> f64 { let mut z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; let sign : bool ; ix = get_high_word (x) ; sign = (ix >> 31) != 0 ; ix &= 0x7fffffff ; if ix >= 0x7ff00000 { return 1.0 / (x * x) ; } if ix >= 0x40000000 { return common (ix , fabs (x) , false , sign) ; } if ix >= 0x38000000 { z = x * x ; r = z * (R00 + z * (R01 + z * (R02 + z * R03))) ; s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * (S04 + z * S05)))) ; z = r / s ; } else { z = x ; } return (0.5 + z) * x ; }
};
}
