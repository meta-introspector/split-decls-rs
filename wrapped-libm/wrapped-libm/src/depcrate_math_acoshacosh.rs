// Generated macro for acosh (function)
macro_rules! Depcrate_math_acoshacosh {
() => {
// Module: crate::math::acosh
// Provides: {"acosh"}
// Dependencies: {}
# [doc = " Inverse hyperbolic cosine (f64)"] # [doc = ""] # [doc = " Calculates the inverse hyperbolic cosine of `x`."] # [doc = " Is defined as `log(x + sqrt(x*x-1))`."] # [doc = " `x` must be a number greater than or equal to 1."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn acosh (x : f64) -> f64 { let u = x . to_bits () ; let e = ((u >> 52) as usize) & 0x7ff ; if e < 0x3ff + 1 { return log1p (x - 1.0 + sqrt ((x - 1.0) * (x - 1.0) + 2.0 * (x - 1.0))) ; } if e < 0x3ff + 26 { return log (2.0 * x - 1.0 / (x + sqrt (x * x - 1.0))) ; } return log (x) + LN2 ; }
};
}
