// Generated macro for f32_ulps_test4 (function)
macro_rules! Depcrate_ulpsf32_ulps_test4 {
() => {
// Module: crate::ulps
// Provides: {"f32_ulps_test4"}
// Dependencies: {}
# [test] fn f32_ulps_test4 () { let x : f32 = f32 :: from_bits (0x63a7f026_u32) ; let y : f32 = f32 :: from_bits (0x63a7f023_u32) ; assert ! (x . ulps (& y) == 3) ; }
};
}
