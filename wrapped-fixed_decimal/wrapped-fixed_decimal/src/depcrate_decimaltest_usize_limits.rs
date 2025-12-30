// Generated macro for test_usize_limits (function)
macro_rules! Depcrate_decimaltest_usize_limits {
() => {
// Module: crate::decimal
// Provides: {"test_usize_limits"}
// Dependencies: {}
# [test] fn test_usize_limits () { for num in & [usize :: MAX , usize :: MIN] { let dec : UnsignedDecimal = (* num) . into () ; let dec_str = dec . to_string () ; assert_eq ! (num . to_string () , dec_str) ; assert_eq ! (dec , UnsignedDecimal :: from_str (& dec_str) . unwrap ()) ; writeable :: assert_writeable_eq ! (dec , dec_str) ; } }
};
}
