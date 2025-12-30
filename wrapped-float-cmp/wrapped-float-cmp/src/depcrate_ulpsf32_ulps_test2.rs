// Generated macro for f32_ulps_test2 (function)
macro_rules! Depcrate_ulpsf32_ulps_test2 {
() => {
// Module: crate::ulps
// Provides: {"f32_ulps_test2"}
// Dependencies: {}
# [test] fn f32_ulps_test2 () { let pzero : f32 = f32 :: from_bits (0x00000000_u32) ; let nzero : f32 = f32 :: from_bits (0x80000000_u32) ; assert_eq ! (pzero . ulps (& nzero) , 1) ; }
};
}
