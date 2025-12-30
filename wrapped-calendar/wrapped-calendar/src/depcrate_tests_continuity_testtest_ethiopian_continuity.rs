// Generated macro for test_ethiopian_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_ethiopian_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_ethiopian_continuity"}
// Dependencies: {}
# [test] fn test_ethiopian_continuity () { use cal :: EthiopianEraStyle :: * ; let date = Date :: try_new_ethiopian (AmeteMihret , - 10 , 1 , 1) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_ethiopian (AmeteMihret , - 300 , 1 , 1) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
