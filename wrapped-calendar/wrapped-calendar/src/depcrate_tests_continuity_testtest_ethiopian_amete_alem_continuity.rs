// Generated macro for test_ethiopian_amete_alem_continuity (function)
macro_rules! Depcrate_tests_continuity_testtest_ethiopian_amete_alem_continuity {
() => {
// Module: crate::tests::continuity_test
// Provides: {"test_ethiopian_amete_alem_continuity"}
// Dependencies: {}
# [test] fn test_ethiopian_amete_alem_continuity () { use cal :: EthiopianEraStyle :: * ; let date = Date :: try_new_ethiopian (AmeteAlem , - 10 , 1 , 1) ; check_continuity (date . unwrap () , 20) ; let date = Date :: try_new_ethiopian (AmeteAlem , - 300 , 1 , 1) ; check_every_250_days (date . unwrap () , 2000) ; }
};
}
