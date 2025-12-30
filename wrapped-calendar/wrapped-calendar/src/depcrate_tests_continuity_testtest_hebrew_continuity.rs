// Generated macro for test_hebrew_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_hebrew_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_hebrew_continuity"}
// Dependencies: {}
# [test] fn test_hebrew_continuity () { let date = Date :: try_new_from_codes (None , - 10 , Month :: new (1) . code () , 1 , cal :: Hebrew) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_from_codes (None , - 300 , Month :: new (1) . code () , 1 , cal :: Hebrew) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
