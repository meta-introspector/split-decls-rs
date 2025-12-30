// Generated macro for log (function)
macro_rules! Depcrate_math_loglog {
() => {
// Module: crate::math::log
// Provides: {"log"}
// Dependencies: {}
# [doc = " The natural logarithm of `x` (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn log (mut x : f64) -> f64 { let x1p54 = f64 :: from_bits (0x4350000000000000) ; let mut ui = x . to_bits () ; let mut hx : u32 = (ui >> 32) as u32 ; let mut k : i32 = 0 ; if (hx < 0x00100000) || ((hx >> 31) != 0) { if ui << 1 == 0 { return - 1. / (x * x) ; } if hx >> 31 != 0 { return (x - x) / 0.0 ; } k -= 54 ; x *= x1p54 ; ui = x . to_bits () ; hx = (ui >> 32) as u32 ; } else if hx >= 0x7ff00000 { return x ; } else if hx == 0x3ff00000 && ui << 32 == 0 { return 0. ; } hx += 0x3ff00000 - 0x3fe6a09e ; k += ((hx >> 20) as i32) - 0x3ff ; hx = (hx & 0x000fffff) + 0x3fe6a09e ; ui = ((hx as u64) << 32) | (ui & 0xffffffff) ; x = f64 :: from_bits (ui) ; let f : f64 = x - 1.0 ; let hfsq : f64 = 0.5 * f * f ; let s : f64 = f / (2.0 + f) ; let z : f64 = s * s ; let w : f64 = z * z ; let t1 : f64 = w * (LG2 + w * (LG4 + w * LG6)) ; let t2 : f64 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7))) ; let r : f64 = t2 + t1 ; let dk : f64 = k as f64 ; s * (hfsq + r) + dk * LN2_LO - hfsq + f + dk * LN2_HI }
};
}
