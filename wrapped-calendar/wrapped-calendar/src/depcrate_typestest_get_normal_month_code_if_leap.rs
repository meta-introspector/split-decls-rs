// Generated macro for test_get_normal_month_code_if_leap (function)
macro_rules! Depcrate_typestest_get_normal_month_code_if_leap {
() => {
// Module: crate::types
// Provides: {"test_get_normal_month_code_if_leap"}
// Dependencies: {}
# [test] fn test_get_normal_month_code_if_leap () { # ! [allow (deprecated)] assert_eq ! (MonthCode :: new_leap (1) . unwrap () . get_normal_if_leap () , MonthCode :: new_normal (1)) ; assert_eq ! (MonthCode :: new_leap (11) . unwrap () . get_normal_if_leap () , MonthCode :: new_normal (11)) ; assert_eq ! (MonthCode :: new_normal (10) . unwrap () . get_normal_if_leap () , None) ; }
};
}
