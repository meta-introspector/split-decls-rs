// Generated macro for exp (function)
macro_rules! Depcrate_math_expexp {
() => {
// Module: crate::math::exp
// Provides: {"exp"}
// Dependencies: {}
# [doc = " Exponential, base *e* (f64)"] # [doc = ""] # [doc = " Calculate the exponential of `x`, that is, *e* raised to the power `x`"] # [doc = " (where *e* is the base of the natural system of logarithms, approximately 2.71828)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn exp (mut x : f64) -> f64 { let x1p1023 = f64 :: from_bits (0x7fe0000000000000) ; let x1p_149 = f64 :: from_bits (0x36a0000000000000) ; let hi : f64 ; let lo : f64 ; let c : f64 ; let xx : f64 ; let y : f64 ; let k : i32 ; let sign : i32 ; let mut hx : u32 ; hx = (x . to_bits () >> 32) as u32 ; sign = (hx >> 31) as i32 ; hx &= 0x7fffffff ; if hx >= 0x4086232b { if x . is_nan () { return x ; } if x > 709.782712893383973096 { x *= x1p1023 ; return x ; } if x < - 708.39641853226410622 { force_eval ! ((- x1p_149 / x) as f32) ; if x < - 745.13321910194110842 { return 0. ; } } } if hx > 0x3fd62e42 { if hx >= 0x3ff0a2b2 { k = (INVLN2 * x + i ! (HALF , sign as usize)) as i32 ; } else { k = 1 - sign - sign ; } hi = x - k as f64 * LN2HI ; lo = k as f64 * LN2LO ; x = hi - lo ; } else if hx > 0x3e300000 { k = 0 ; hi = x ; lo = 0. ; } else { force_eval ! (x1p1023 + x) ; return 1. + x ; } xx = x * x ; c = x - xx * (P1 + xx * (P2 + xx * (P3 + xx * (P4 + xx * P5)))) ; y = 1. + (x * c / (2. - c) - lo + hi) ; if k == 0 { y } else { scalbn (y , k) } }
};
}
