// Generated macro for tgamma (function)
macro_rules! Depcrate_math_tgammatgamma {
() => {
// Module: crate::math::tgamma
// Provides: {"tgamma"}
// Dependencies: {}
# [doc = " The [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn tgamma (mut x : f64) -> f64 { let u : u64 = x . to_bits () ; let absx : f64 ; let mut y : f64 ; let mut dy : f64 ; let mut z : f64 ; let mut r : f64 ; let ix : u32 = ((u >> 32) as u32) & 0x7fffffff ; let sign : bool = (u >> 63) != 0 ; if ix >= 0x7ff00000 { return x + f64 :: INFINITY ; } if ix < ((0x3ff - 54) << 20) { return 1.0 / x ; } if x == floor (x) { if sign { return 0.0 / 0.0 ; } if x <= FACT . len () as f64 { return i ! (FACT , (x as usize) - 1) ; } } if ix >= 0x40670000 { if sign { let x1p_126 = f64 :: from_bits (0x3810000000000000) ; force_eval ! ((x1p_126 / x) as f32) ; if floor (x) * 0.5 == floor (x * 0.5) { return 0.0 ; } else { return - 0.0 ; } } let x1p1023 = f64 :: from_bits (0x7fe0000000000000) ; x *= x1p1023 ; return x ; } absx = if sign { - x } else { x } ; y = absx + GMHALF ; if absx > GMHALF { dy = y - absx ; dy -= GMHALF ; } else { dy = y - GMHALF ; dy -= absx ; } z = absx - 0.5 ; r = s (absx) * exp (- y) ; if x < 0.0 { r = - PI / (sinpi (absx) * absx * r) ; dy = - dy ; z = - z ; } r += dy * (GMHALF + 0.5) * r / y ; z = pow (y , 0.5 * z) ; y = r * z * z ; return y ; }
};
}
