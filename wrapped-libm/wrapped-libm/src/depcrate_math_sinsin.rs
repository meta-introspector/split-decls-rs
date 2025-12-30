// Generated macro for sin (function)
macro_rules! Depcrate_math_sinsin {
() => {
// Module: crate::math::sin
// Provides: {"sin"}
// Dependencies: {}
# [doc = " The sine of `x` (f64)."] # [doc = ""] # [doc = " `x` is specified in radians."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sin (x : f64) -> f64 { let x1p120 = f64 :: from_bits (0x4770000000000000) ; let ix = (f64 :: to_bits (x) >> 32) as u32 & 0x7fffffff ; if ix <= 0x3fe921fb { if ix < 0x3e500000 { if ix < 0x00100000 { force_eval ! (x / x1p120) ; } else { force_eval ! (x + x1p120) ; } return x ; } return k_sin (x , 0.0 , 0) ; } if ix >= 0x7ff00000 { return x - x ; } let (n , y0 , y1) = rem_pio2 (x) ; match n & 3 { 0 => k_sin (y0 , y1 , 1) , 1 => k_cos (y0 , y1) , 2 => - k_sin (y0 , y1 , 1) , _ => - k_cos (y0 , y1) , } }
};
}
