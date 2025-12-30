// Generated macro for f64_ulps_test4 (function)
macro_rules! Depcrate_ulpsf64_ulps_test4 {
() => {
// Module: crate::ulps
// Provides: {"f64_ulps_test4"}
// Dependencies: {}
# [test] fn f64_ulps_test4 () { let x : f64 = f64 :: from_bits (0xd017f6cc63a7f026_u64) ; let y : f64 = f64 :: from_bits (0xd017f6cc63a7f023_u64) ; assert ! (x . ulps (& y) == - 3) ; }
};
}
