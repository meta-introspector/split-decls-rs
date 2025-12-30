// Generated macro for acos (function)
macro_rules! Depcrate_math_acosacos {
() => {
// Module: crate::math::acos
// Provides: {"acos"}
// Dependencies: {}
# [doc = " Arccosine (f64)"] # [doc = ""] # [doc = " Computes the inverse cosine (arc cosine) of the input value."] # [doc = " Arguments must be in the range -1 to 1."] # [doc = " Returns values in radians, in the range of 0 to pi."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn acos (x : f64) -> f64 { let x1p_120f = f64 :: from_bits (0x3870000000000000) ; let z : f64 ; let w : f64 ; let s : f64 ; let c : f64 ; let df : f64 ; let hx : u32 ; let ix : u32 ; hx = (x . to_bits () >> 32) as u32 ; ix = hx & 0x7fffffff ; if ix >= 0x3ff00000 { let lx : u32 = x . to_bits () as u32 ; if ((ix - 0x3ff00000) | lx) == 0 { if (hx >> 31) != 0 { return 2. * PIO2_HI + x1p_120f ; } return 0. ; } return 0. / (x - x) ; } if ix < 0x3fe00000 { if ix <= 0x3c600000 { return PIO2_HI + x1p_120f ; } return PIO2_HI - (x - (PIO2_LO - x * r (x * x))) ; } if (hx >> 31) != 0 { z = (1.0 + x) * 0.5 ; s = sqrt (z) ; w = r (z) * s - PIO2_LO ; return 2. * (PIO2_HI - (s + w)) ; } z = (1.0 - x) * 0.5 ; s = sqrt (z) ; df = f64 :: from_bits (s . to_bits () & 0xff_ff_ff_ff_00_00_00_00) ; c = (z - df * df) / (s + df) ; w = r (z) * s + c ; 2. * (df + w) }
};
}
