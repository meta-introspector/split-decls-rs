// Generated macro for test_pad_end_bounds (function)
macro_rules! Depcrate_decimaltest_pad_end_bounds {
() => {
// Module: crate::decimal
// Provides: {"test_pad_end_bounds"}
// Dependencies: {}
# [test] fn test_pad_end_bounds () { let mut dec = UnsignedDecimal :: from_str ("299792.458") . unwrap () ; let max_fractional_digits = - (i16 :: MIN as isize) as usize ; dec . pad_end (i16 :: MIN + 1) ; assert_eq ! (max_fractional_digits - 1 , dec . to_string () . split_once ('.') . unwrap () . 1 . len ()) ; dec . pad_end (i16 :: MIN) ; assert_eq ! (max_fractional_digits , dec . to_string () . split_once ('.') . unwrap () . 1 . len ()) ; }
};
}
