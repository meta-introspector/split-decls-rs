// Generated macro for test_korean_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_korean_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_korean_continuity"}
// Dependencies: {}
# [test] fn test_korean_continuity () { let cal = cal :: KoreanTraditional :: new () ; let date = Date :: try_new_from_codes (None , - 10 , Month :: new (1) . code () , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_from_codes (None , - 300 , Month :: new (1) . code () , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; let date = Date :: try_new_from_codes (None , 1900 , Month :: new (1) . code () , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_from_codes (None , 2100 , Month :: new (1) . code () , 1 , cal) ; check_continuity (date . unwrap () , 20) ; }
};
}
