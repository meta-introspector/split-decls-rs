// Generated macro for asin (function)
macro_rules! Depcrate_math_asinasin {
() => {
// Module: crate::math::asin
// Provides: {"asin"}
// Dependencies: {}
# [doc = " Arcsine (f64)"] # [doc = ""] # [doc = " Computes the inverse sine (arc sine) of the argument `x`."] # [doc = " Arguments to asin must be in the range -1 to 1."] # [doc = " Returns values in radians, in the range of -pi/2 to pi/2."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn asin (mut x : f64) -> f64 { let z : f64 ; let r : f64 ; let s : f64 ; let hx : u32 ; let ix : u32 ; hx = get_high_word (x) ; ix = hx & 0x7fffffff ; if ix >= 0x3ff00000 { let lx : u32 ; lx = get_low_word (x) ; if ((ix - 0x3ff00000) | lx) == 0 { return x * PIO2_HI + f64 :: from_bits (0x3870000000000000) ; } else { return 0.0 / (x - x) ; } } if ix < 0x3fe00000 { if (0x00100000 .. 0x3e500000) . contains (& ix) { return x ; } else { return x + x * comp_r (x * x) ; } } z = (1.0 - fabs (x)) * 0.5 ; s = sqrt (z) ; r = comp_r (z) ; if ix >= 0x3fef3333 { x = PIO2_HI - (2. * (s + s * r) - PIO2_LO) ; } else { let f : f64 ; let c : f64 ; f = with_set_low_word (s , 0) ; c = (z - f * f) / (s + f) ; x = 0.5 * PIO2_HI - (2.0 * s * r - (PIO2_LO - 2.0 * c) - (0.5 * PIO2_HI - 2.0 * f)) ; } if hx >> 31 != 0 { - x } else { x } }
};
}
