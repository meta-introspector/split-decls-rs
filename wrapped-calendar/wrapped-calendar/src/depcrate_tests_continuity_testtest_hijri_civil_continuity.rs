// Generated macro for test_hijri_civil_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_hijri_civil_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_hijri_civil_continuity"}
// Dependencies: {}
# [test] fn test_hijri_civil_continuity () { let cal = cal :: Hijri :: new_tabular (cal :: hijri :: TabularAlgorithmLeapYears :: TypeII , cal :: hijri :: TabularAlgorithmEpoch :: Friday ,) ; let date = Date :: try_new_hijri_with_calendar (- 10 , 1 , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_hijri_with_calendar (- 300 , 1 , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
