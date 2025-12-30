// Generated macro for test_i16_sub_unsigned (function)
macro_rules! Depcrate_opstest_i16_sub_unsigned {
() => {
// Module: crate::ops
// Provides: {"test_i16_sub_unsigned"}
// Dependencies: {}
# [test] fn test_i16_sub_unsigned () { assert_eq ! (i16_sub_unsigned (5 , 0) , 5) ; assert_eq ! (i16_sub_unsigned (5 , 4) , 1) ; assert_eq ! (i16_sub_unsigned (5 , 5) , 0) ; assert_eq ! (i16_sub_unsigned (5 , 6) , - 1) ; assert_eq ! (i16_sub_unsigned (0 , 0) , 0) ; assert_eq ! (i16_sub_unsigned (0 , 1) , - 1) ; assert_eq ! (i16_sub_unsigned (- 5 , 0) , - 5) ; assert_eq ! (i16_sub_unsigned (- 5 , 1) , - 6) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , 0) , i16 :: MAX) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , 1) , i16 :: MAX - 1) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , i16 :: MAX as u16 - 1) , 1) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , i16 :: MAX as u16) , 0) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , i16 :: MAX as u16 + 1) , - 1) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , u16 :: MAX - 1) , i16 :: MIN + 1) ; assert_eq ! (i16_sub_unsigned (i16 :: MAX , u16 :: MAX) , i16 :: MIN) ; }
};
}
