// Generated macro for f32_ulps_test3 (function)
macro_rules! Depcrate_ulpsf32_ulps_test3 {
() => {
// Module: crate::ulps
// Provides: {"f32_ulps_test3"}
// Dependencies: {}
# [test] fn f32_ulps_test3 () { let pinf : f32 = f32 :: from_bits (0x7f800000_u32) ; let ninf : f32 = f32 :: from_bits (0xff800000_u32) ; assert_eq ! (pinf . ulps (& ninf) , - 16777215) ; }
};
}
