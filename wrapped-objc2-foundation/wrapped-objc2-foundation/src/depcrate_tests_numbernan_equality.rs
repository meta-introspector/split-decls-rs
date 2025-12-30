// Generated macro for nan_equality (function)
macro_rules! Depcrate_tests_numbernan_equality {
() => {
// Module: crate::tests::number
// Provides: {"nan_equality"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "GNUStep handles NaNs differently")] fn nan_equality () { let nan = NSNumber :: new_f32 (f32 :: NAN) ; let nan2 = NSNumber :: new_f32 (f32 :: NAN) ; let neg_nan = NSNumber :: new_f32 (- f32 :: NAN) ; assert_eq ! (nan , nan) ; assert_eq ! (nan , nan2) ; assert_eq ! (neg_nan , neg_nan) ; assert_eq ! (nan , neg_nan) ; }
};
}
