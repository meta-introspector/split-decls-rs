// Generated macro for f32_ulps_test5 (function)
macro_rules! Depcrate_ulpsf32_ulps_test5 {
() => {
// Module: crate::ulps
// Provides: {"f32_ulps_test5"}
// Dependencies: {}
# [test] fn f32_ulps_test5 () { let x : f32 = 2.0 ; let ulps : i32 = x . to_bits () as i32 ; let x2 : f32 = < f32 > :: from_bits (ulps as u32) ; assert_eq ! (x , x2) ; }
};
}
