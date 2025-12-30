// Generated macro for test_u128_limits (function)
macro_rules! Depcrate_decimaltest_u128_limits {
() => {
// Module: crate::decimal
// Provides: {"test_u128_limits"}
// Dependencies: {}
# [test] fn test_u128_limits () { for num in & [u128 :: MAX , u128 :: MIN] { let dec : UnsignedDecimal = (* num) . into () ; let dec_str = dec . to_string () ; assert_eq ! (num . to_string () , dec_str) ; assert_eq ! (dec , UnsignedDecimal :: from_str (& dec_str) . unwrap ()) ; writeable :: assert_writeable_eq ! (dec , dec_str) ; } }
};
}
