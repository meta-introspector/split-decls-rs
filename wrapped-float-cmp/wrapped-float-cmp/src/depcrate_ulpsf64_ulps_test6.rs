// Generated macro for f64_ulps_test6 (function)
macro_rules! Depcrate_ulpsf64_ulps_test6 {
() => {
// Module: crate::ulps
// Provides: {"f64_ulps_test6"}
// Dependencies: {}
# [test] fn f64_ulps_test6 () { let negzero : f64 = - 0. ; let zero : f64 = 0. ; assert_eq ! (negzero . next () , zero) ; assert_eq ! (zero . prev () , negzero) ; assert ! (negzero . prev () < 0.0) ; assert ! (zero . next () > 0.0) ; }
};
}
