// Generated macro for test_pad_start_bounds (function)
macro_rules! Depcrate_signed_decimaltest_pad_start_bounds {
() => {
// Module: crate::signed_decimal
// Provides: {"test_pad_start_bounds"}
// Dependencies: {}
# [test] fn test_pad_start_bounds () { let mut dec = Decimal :: from_str ("299792.458") . unwrap () ; let max_integer_digits = i16 :: MAX as usize + 1 ; dec . absolute . pad_start (i16 :: MAX - 1) ; assert_eq ! (max_integer_digits - 2 , dec . to_string () . split_once ('.') . unwrap () . 0 . len ()) ; dec . absolute . pad_start (i16 :: MAX) ; assert_eq ! (max_integer_digits - 1 , dec . to_string () . split_once ('.') . unwrap () . 0 . len ()) ; }
};
}
