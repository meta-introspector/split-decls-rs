// Generated macro for y1 (function)
macro_rules! Depcrate_math_j1y1 {
() => {
// Module: crate::math::j1
// Provides: {"y1"}
// Dependencies: {}
# [doc = " First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn y1 (x : f64) -> f64 { let z : f64 ; let u : f64 ; let v : f64 ; let ix : u32 ; let lx : u32 ; ix = get_high_word (x) ; lx = get_low_word (x) ; if (ix << 1) | lx == 0 { return - 1.0 / 0.0 ; } if ix >> 31 != 0 { return 0.0 / 0.0 ; } if ix >= 0x7ff00000 { return 1.0 / x ; } if ix >= 0x40000000 { return common (ix , x , true , false) ; } if ix < 0x3c900000 { return - TPI / x ; } z = x * x ; u = U0 [0] + z * (U0 [1] + z * (U0 [2] + z * (U0 [3] + z * U0 [4]))) ; v = 1.0 + z * (V0 [0] + z * (V0 [1] + z * (V0 [2] + z * (V0 [3] + z * V0 [4])))) ; return x * (u / v) + TPI * (j1 (x) * log (x) - 1.0 / x) ; }
};
}
