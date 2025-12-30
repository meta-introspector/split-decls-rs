// Generated macro for f64_ulps_test5 (function)
macro_rules! Depcrate_ulpsf64_ulps_test5 {
() => {
// Module: crate::ulps
// Provides: {"f64_ulps_test5"}
// Dependencies: {}
# [test] fn f64_ulps_test5 () { let x : f64 = 2.0 ; let ulps : i64 = x . to_bits () as i64 ; let x2 : f64 = < f64 > :: from_bits (ulps as u64) ; assert_eq ! (x , x2) ; }
};
}
