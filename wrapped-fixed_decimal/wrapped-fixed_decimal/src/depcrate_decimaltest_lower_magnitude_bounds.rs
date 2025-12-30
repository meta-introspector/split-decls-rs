// Generated macro for test_lower_magnitude_bounds (function)
macro_rules! Depcrate_decimaltest_lower_magnitude_bounds {
() => {
// Module: crate::decimal
// Provides: {"test_lower_magnitude_bounds"}
// Dependencies: {}
# [test] fn test_lower_magnitude_bounds () { let mut dec : UnsignedDecimal = 98765u32 . into () ; assert_eq ! (dec . lower_magnitude , 0) ; dec . multiply_pow10 (i16 :: MIN) ; assert_eq ! (dec . lower_magnitude , i16 :: MIN) ; assert_eq ! (dec . nonzero_magnitude_end () , i16 :: MIN) ; let dec_backup = dec . clone () ; dec . multiply_pow10 (- 1) ; assert ! (dec . is_zero ()) ; assert_ne ! (dec , dec_backup) ; let dec_roundtrip = UnsignedDecimal :: from_str (& dec . to_string ()) . unwrap () ; assert_eq ! (dec , dec_roundtrip) ; }
};
}
