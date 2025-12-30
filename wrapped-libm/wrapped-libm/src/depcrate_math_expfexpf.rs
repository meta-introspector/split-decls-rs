// Generated macro for expf (function)
macro_rules! Depcrate_math_expfexpf {
() => {
// Module: crate::math::expf
// Provides: {"expf"}
// Dependencies: {}
# [doc = " Exponential, base *e* (f32)"] # [doc = ""] # [doc = " Calculate the exponential of `x`, that is, *e* raised to the power `x`"] # [doc = " (where *e* is the base of the natural system of logarithms, approximately 2.71828)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn expf (mut x : f32) -> f32 { let x1p127 = f32 :: from_bits (0x7f000000) ; let x1p_126 = f32 :: from_bits (0x800000) ; let mut hx = x . to_bits () ; let sign = (hx >> 31) as i32 ; let signb : bool = sign != 0 ; hx &= 0x7fffffff ; if hx >= 0x42aeac50 { if hx > 0x7f800000 { return x ; } if (hx >= 0x42b17218) && (! signb) { x *= x1p127 ; return x ; } if signb { force_eval ! (- x1p_126 / x) ; if hx >= 0x42cff1b5 { return 0. ; } } } let k : i32 ; let hi : f32 ; let lo : f32 ; if hx > 0x3eb17218 { if hx > 0x3f851592 { k = (INV_LN2 * x + i ! (HALF , sign as usize)) as i32 ; } else { k = 1 - sign - sign ; } let kf = k as f32 ; hi = x - kf * LN2_HI ; lo = kf * LN2_LO ; x = hi - lo ; } else if hx > 0x39000000 { k = 0 ; hi = x ; lo = 0. ; } else { force_eval ! (x1p127 + x) ; return 1. + x ; } let xx = x * x ; let c = x - xx * (P1 + xx * P2) ; let y = 1. + (x * c / (2. - c) - lo + hi) ; if k == 0 { y } else { scalbnf (y , k) } }
};
}
