// Generated macro for asinh (function)
macro_rules! Depcrate_math_asinhasinh {
() => {
// Module: crate::math::asinh
// Provides: {"asinh"}
// Dependencies: {}
# [doc = " Inverse hyperbolic sine (f64)"] # [doc = ""] # [doc = " Calculates the inverse hyperbolic sine of `x`."] # [doc = " Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn asinh (mut x : f64) -> f64 { let mut u = x . to_bits () ; let e = ((u >> 52) as usize) & 0x7ff ; let sign = (u >> 63) != 0 ; u &= (! 0) >> 1 ; x = f64 :: from_bits (u) ; if e >= 0x3ff + 26 { x = log (x) + LN2 ; } else if e >= 0x3ff + 1 { x = log (2.0 * x + 1.0 / (sqrt (x * x + 1.0) + x)) ; } else if e >= 0x3ff - 26 { x = log1p (x + x * x / (sqrt (x * x + 1.0) + 1.0)) ; } else { let x1p120 = f64 :: from_bits (0x4770000000000000) ; force_eval ! (x + x1p120) ; } if sign { - x } else { x } }
};
}
