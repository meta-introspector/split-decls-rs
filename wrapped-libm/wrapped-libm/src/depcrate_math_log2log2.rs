// Generated macro for log2 (function)
macro_rules! Depcrate_math_log2log2 {
() => {
// Module: crate::math::log2
// Provides: {"log2"}
// Dependencies: {}
# [doc = " The base 2 logarithm of `x` (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn log2 (mut x : f64) -> f64 { let x1p54 = f64 :: from_bits (0x4350000000000000) ; let mut ui : u64 = x . to_bits () ; let hfsq : f64 ; let f : f64 ; let s : f64 ; let z : f64 ; let r : f64 ; let mut w : f64 ; let t1 : f64 ; let t2 : f64 ; let y : f64 ; let mut hi : f64 ; let lo : f64 ; let mut val_hi : f64 ; let mut val_lo : f64 ; let mut hx : u32 ; let mut k : i32 ; hx = (ui >> 32) as u32 ; k = 0 ; if hx < 0x00100000 || (hx >> 31) > 0 { if ui << 1 == 0 { return - 1. / (x * x) ; } if (hx >> 31) > 0 { return (x - x) / 0.0 ; } k -= 54 ; x *= x1p54 ; ui = x . to_bits () ; hx = (ui >> 32) as u32 ; } else if hx >= 0x7ff00000 { return x ; } else if hx == 0x3ff00000 && ui << 32 == 0 { return 0. ; } hx += 0x3ff00000 - 0x3fe6a09e ; k += (hx >> 20) as i32 - 0x3ff ; hx = (hx & 0x000fffff) + 0x3fe6a09e ; ui = ((hx as u64) << 32) | (ui & 0xffffffff) ; x = f64 :: from_bits (ui) ; f = x - 1.0 ; hfsq = 0.5 * f * f ; s = f / (2.0 + f) ; z = s * s ; w = z * z ; t1 = w * (LG2 + w * (LG4 + w * LG6)) ; t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7))) ; r = t2 + t1 ; hi = f - hfsq ; ui = hi . to_bits () ; ui &= (- 1i64 as u64) << 32 ; hi = f64 :: from_bits (ui) ; lo = f - hi - hfsq + s * (hfsq + r) ; val_hi = hi * IVLN2HI ; val_lo = (lo + hi) * IVLN2LO + lo * IVLN2HI ; y = k . into () ; w = y + val_hi ; val_lo += (y - w) + val_hi ; val_hi = w ; val_lo + val_hi }
};
}
