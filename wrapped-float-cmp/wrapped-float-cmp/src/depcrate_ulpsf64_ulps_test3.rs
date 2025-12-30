// Generated macro for f64_ulps_test3 (function)
macro_rules! Depcrate_ulpsf64_ulps_test3 {
() => {
// Module: crate::ulps
// Provides: {"f64_ulps_test3"}
// Dependencies: {}
# [test] fn f64_ulps_test3 () { let pinf : f64 = f64 :: from_bits (0x7f80000000000000_u64) ; let ninf : f64 = f64 :: from_bits (0xff80000000000000_u64) ; assert_eq ! (pinf . ulps (& ninf) , - 72057594037927935) ; }
};
}
