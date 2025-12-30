// Generated macro for test_chinese_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_chinese_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_chinese_continuity"}
// Dependencies: {}
# [test] fn test_chinese_continuity () { let cal = crate :: cal :: ChineseTraditional :: new () ; let date = Date :: try_new_from_codes (None , - 10 , Month :: new (1) . code () , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_from_codes (None , - 300 , Month :: new (1) . code () , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; let date = Date :: try_new_from_codes (None , - 10000 , Month :: new (1) . code () , 1 , cal) ; check_every_250_days (date . unwrap () , 2000) ; let date = Date :: try_new_from_codes (None , 1899 , Month :: new (1) . code () , 1 , cal) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_from_codes (None , 2099 , Month :: new (1) . code () , 1 , cal) ; check_continuity (date . unwrap () , 20) ; }
};
}
