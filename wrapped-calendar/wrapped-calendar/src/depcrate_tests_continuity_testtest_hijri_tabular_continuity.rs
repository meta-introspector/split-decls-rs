// Generated macro for test_hijri_tabular_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_hijri_tabular_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_hijri_tabular_continuity"}
// Dependencies: {}
# [test] fn test_hijri_tabular_continuity () { let cal = cal :: Hijri :: new_tabular (cal :: hijri :: TabularAlgorithmLeapYears :: TypeII , cal :: hijri :: TabularAlgorithmEpoch :: Thursday ,) ; let date = Date :: try_new_hijri_with_calendar (- 10 , 1 , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_hijri_with_calendar (- 300 , 1 , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
