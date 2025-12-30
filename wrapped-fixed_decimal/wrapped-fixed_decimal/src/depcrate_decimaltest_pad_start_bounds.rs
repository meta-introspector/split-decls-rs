// Generated macro for test_pad_start_bounds (function)
macro_rules! Depcrate_decimaltest_pad_start_bounds {
() => {
// Module: crate::decimal
// Provides: {"test_pad_start_bounds"}
// Dependencies: {}
# [test] fn test_pad_start_bounds () { let mut dec = UnsignedDecimal :: from_str ("299792.458") . unwrap () ; let max_integer_digits = i16 :: MAX as usize + 1 ; dec . pad_start (i16 :: MAX - 1) ; assert_eq ! (max_integer_digits - 2 , dec . to_string () . split_once ('.') . unwrap () . 0 . len ()) ; dec . pad_start (i16 :: MAX) ; assert_eq ! (max_integer_digits - 1 , dec . to_string () . split_once ('.') . unwrap () . 0 . len ()) ; }
};
}
