// Generated macro for test_isize_limits (function)
macro_rules! Depcrate_signed_decimaltest_isize_limits {
() => {
// Module: crate::signed_decimal
// Provides: {"test_isize_limits"}
// Dependencies: {}
# [test] fn test_isize_limits () { for num in & [isize :: MAX , isize :: MIN] { let dec : Decimal = (* num) . into () ; let dec_str = dec . to_string () ; assert_eq ! (num . to_string () , dec_str) ; assert_eq ! (dec , Decimal :: from_str (& dec_str) . unwrap ()) ; writeable :: assert_writeable_eq ! (dec , dec_str) ; } }
};
}
