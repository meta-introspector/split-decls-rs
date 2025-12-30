// Generated macro for test_i16_abs_sub (function)
macro_rules! Depcrate_opstest_i16_abs_sub {
() => {
// Module: crate::ops
// Provides: {"test_i16_abs_sub"}
// Dependencies: {}
# [test] fn test_i16_abs_sub () { assert_eq ! (i16_abs_sub (5 , - 1) , 6) ; assert_eq ! (i16_abs_sub (5 , 0) , 5) ; assert_eq ! (i16_abs_sub (5 , 4) , 1) ; assert_eq ! (i16_abs_sub (5 , 5) , 0) ; assert_eq ! (i16_abs_sub (0 , - 1) , 1) ; assert_eq ! (i16_abs_sub (0 , 0) , 0) ; assert_eq ! (i16_abs_sub (- 5 , - 5) , 0) ; assert_eq ! (i16_abs_sub (- 5 , - 6) , 1) ; assert_eq ! (i16_abs_sub (i16 :: MAX , i16 :: MIN) , u16 :: MAX) ; assert_eq ! (i16_abs_sub (i16 :: MAX , i16 :: MIN + 1) , u16 :: MAX - 1) ; assert_eq ! (i16_abs_sub (i16 :: MAX , - 1) , i16 :: MAX as u16 + 1) ; assert_eq ! (i16_abs_sub (i16 :: MAX , 0) , i16 :: MAX as u16) ; assert_eq ! (i16_abs_sub (i16 :: MAX , 1) , i16 :: MAX as u16 - 1) ; assert_eq ! (i16_abs_sub (i16 :: MAX , i16 :: MAX - 1) , 1) ; assert_eq ! (i16_abs_sub (i16 :: MAX , i16 :: MAX) , 0) ; }
};
}
