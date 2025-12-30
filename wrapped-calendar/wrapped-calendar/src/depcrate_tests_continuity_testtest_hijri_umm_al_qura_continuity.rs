// Generated macro for test_hijri_umm_al_qura_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_hijri_umm_al_qura_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_hijri_umm_al_qura_continuity"}
// Dependencies: {}
# [test] fn test_hijri_umm_al_qura_continuity () { # [cfg (feature = "logging")] let _ = simple_logger :: SimpleLogger :: new () . env () . init () ; let cal = cal :: Hijri :: new_umm_al_qura () ; let date = Date :: try_new_hijri_with_calendar (- 10 , 1 , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_hijri_with_calendar (1290 , 1 , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_hijri_with_calendar (1590 , 1 , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_hijri_with_calendar (- 300 , 1 , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
