// Generated macro for test_set_max_position (function)
macro_rules! Depcrate_signed_decimaltest_set_max_position {
() => {
// Module: crate::signed_decimal
// Provides: {"test_set_max_position"}
// Dependencies: {}
# [test] fn test_set_max_position () { let mut dec = Decimal :: from (1000u32) ; assert_eq ! ("1000" , dec . to_string ()) ; dec . absolute . set_max_position (2) ; assert_eq ! ("00" , dec . to_string ()) ; dec . absolute . set_max_position (0) ; assert_eq ! ("0" , dec . to_string ()) ; dec . absolute . set_max_position (3) ; assert_eq ! ("000" , dec . to_string ()) ; let mut dec = Decimal :: from_str ("0.456") . unwrap () ; assert_eq ! ("0.456" , dec . to_string ()) ; dec . absolute . set_max_position (0) ; assert_eq ! ("0.456" , dec . to_string ()) ; dec . absolute . set_max_position (- 1) ; assert_eq ! ("0.056" , dec . to_string ()) ; dec . absolute . set_max_position (- 2) ; assert_eq ! ("0.006" , dec . to_string ()) ; dec . absolute . set_max_position (- 3) ; assert_eq ! ("0.000" , dec . to_string ()) ; dec . absolute . set_max_position (- 4) ; assert_eq ! ("0.0000" , dec . to_string ()) ; let mut dec = Decimal :: from_str ("100.01") . unwrap () ; dec . absolute . set_max_position (1) ; assert_eq ! ("0.01" , dec . to_string ()) ; }
};
}
