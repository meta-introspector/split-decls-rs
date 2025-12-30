// Generated macro for f64_eq (function)
macro_rules! Depcrate_utilsf64_eq {
() => {
// Module: crate::utils
// Provides: {"f64_eq"}
// Dependencies: {}
pub (crate) fn f64_eq (left : f64 , right : f64) -> bool { left == right || fabs (left - right) <= f64 :: EPSILON }
};
}
