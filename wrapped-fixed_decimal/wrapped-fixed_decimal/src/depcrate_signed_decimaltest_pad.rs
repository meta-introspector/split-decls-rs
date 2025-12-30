// Generated macro for test_pad (function)
macro_rules! Depcrate_signed_decimaltest_pad {
() => {
// Module: crate::signed_decimal
// Provides: {"test_pad"}
// Dependencies: {}
# [test] fn test_pad () { let mut dec = Decimal :: from_str ("-0.42") . unwrap () ; assert_eq ! ("-0.42" , dec . to_string ()) ; dec . absolute . pad_start (1) ; assert_eq ! ("-0.42" , dec . to_string ()) ; dec . absolute . pad_start (4) ; assert_eq ! ("-0000.42" , dec . to_string ()) ; dec . absolute . pad_start (2) ; assert_eq ! ("-00.42" , dec . to_string ()) ; }
};
}
