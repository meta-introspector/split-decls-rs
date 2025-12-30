// Generated macro for test_roc_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_roc_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_roc_continuity"}
// Dependencies: {}
# [test] fn test_roc_continuity () { let date = Date :: try_new_roc (- 10 , 1 , 1) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_roc (- 300 , 1 , 1) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
