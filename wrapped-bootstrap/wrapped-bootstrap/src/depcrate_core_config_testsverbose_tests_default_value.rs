// Generated macro for verbose_tests_default_value (function)
macro_rules! Depcrate_core_config_testsverbose_tests_default_value {
() => {
// Module: crate::core::config::tests
// Provides: {"verbose_tests_default_value"}
// Dependencies: {}
# [test] fn verbose_tests_default_value () { let config = Config :: parse (Flags :: parse (& ["build" . into () , "compiler" . into ()])) ; assert_eq ! (config . verbose_tests , false) ; let config = Config :: parse (Flags :: parse (& ["build" . into () , "compiler" . into () , "-v" . into ()])) ; assert_eq ! (config . verbose_tests , true) ; }
};
}
