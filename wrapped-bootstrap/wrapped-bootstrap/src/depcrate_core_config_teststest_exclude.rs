// Generated macro for test_exclude (function)
macro_rules! Depcrate_core_config_teststest_exclude {
() => {
// Module: crate::core::config::tests
// Provides: {"test_exclude"}
// Dependencies: {}
# [test] fn test_exclude () { let exclude_path = "compiler" ; let config = parse (& format ! ("build.exclude=[\"{}\"]" , exclude_path)) ; let first_excluded = config . skip . first () . expect ("Expected at least one excluded path") . to_str () . expect ("Failed to convert excluded path to string") ; assert_eq ! (first_excluded , exclude_path) ; }
};
}
