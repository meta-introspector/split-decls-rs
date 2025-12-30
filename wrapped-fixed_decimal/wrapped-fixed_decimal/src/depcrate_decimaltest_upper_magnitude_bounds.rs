// Generated macro for test_upper_magnitude_bounds (function)
macro_rules! Depcrate_decimaltest_upper_magnitude_bounds {
() => {
// Module: crate::decimal
// Provides: {"test_upper_magnitude_bounds"}
// Dependencies: {}
# [test] fn test_upper_magnitude_bounds () { let mut dec : UnsignedDecimal = 98765u32 . into () ; assert_eq ! (dec . upper_magnitude , 4) ; dec . multiply_pow10 (i16 :: MAX - 4) ; assert_eq ! (dec . upper_magnitude , i16 :: MAX) ; assert_eq ! (dec . nonzero_magnitude_start () , i16 :: MAX) ; let dec_backup = dec . clone () ; dec . multiply_pow10 (1) ; assert ! (dec . is_zero ()) ; assert_ne ! (dec , dec_backup , "Value should be unchanged on failure") ; let dec_roundtrip = UnsignedDecimal :: from_str (& dec . to_string ()) . unwrap () ; assert_eq ! (dec , dec_roundtrip) ; }
};
}
