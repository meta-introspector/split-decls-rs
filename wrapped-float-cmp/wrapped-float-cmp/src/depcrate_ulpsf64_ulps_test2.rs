// Generated macro for f64_ulps_test2 (function)
macro_rules! Depcrate_ulpsf64_ulps_test2 {
() => {
// Module: crate::ulps
// Provides: {"f64_ulps_test2"}
// Dependencies: {}
# [test] fn f64_ulps_test2 () { let pzero : f64 = f64 :: from_bits (0x0000000000000000_u64) ; let nzero : f64 = f64 :: from_bits (0x8000000000000000_u64) ; assert_eq ! (pzero . ulps (& nzero) , 1) ; }
};
}
