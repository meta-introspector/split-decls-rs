// Generated macro for gh23_lossless_integer_max_min_to_float (function)
macro_rules! Depcrate_testgh23_lossless_integer_max_min_to_float {
() => {
// Module: crate::test
// Provides: {"gh23_lossless_integer_max_min_to_float"}
// Dependencies: {}
# [test] fn gh23_lossless_integer_max_min_to_float () { assert_eq ! (Ok (u8 :: MAX) , super :: u8 (255f32)) ; assert_eq ! (Ok (u16 :: MAX) , super :: u16 (65_535f32)) ; assert_eq ! (Ok (u8 :: MAX) , super :: u8 (255f64)) ; assert_eq ! (Ok (u16 :: MAX) , super :: u16 (65_535f64)) ; assert_eq ! (Ok (u32 :: MAX) , super :: u32 (4_294_967_295f64)) ; assert_eq ! (Ok (i8 :: MIN) , super :: i8 (- 128f32)) ; assert_eq ! (Ok (i16 :: MIN) , super :: i16 (- 32_768f32)) ; assert_eq ! (Ok (i8 :: MIN) , super :: i8 (- 128f64)) ; assert_eq ! (Ok (i16 :: MIN) , super :: i16 (- 32_768f64)) ; assert_eq ! (Ok (i32 :: MIN) , super :: i32 (- 2_147_483_648f64)) ; }
};
}
