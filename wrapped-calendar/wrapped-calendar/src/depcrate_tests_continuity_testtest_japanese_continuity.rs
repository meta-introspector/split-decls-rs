// Generated macro for test_japanese_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_japanese_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_japanese_continuity"}
// Dependencies: {}
# [test] fn test_japanese_continuity () { let cal = cal :: Japanese :: new () ; let cal = Ref (& cal) ; let date = Date :: try_new_japanese_with_calendar ("heisei" , 20 , 1 , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_japanese_with_calendar ("bce" , 500 , 1 , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
