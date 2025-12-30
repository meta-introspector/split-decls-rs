// Generated macro for test_parse_value_from_args (function)
macro_rules! Depcrate_utils_build_stamp_tests_shared_helpers_teststest_parse_value_from_args {
() => {
// Module: crate::utils::build_stamp::tests::shared_helpers_tests
// Provides: {"test_parse_value_from_args"}
// Dependencies: {}
# [test] fn test_parse_value_from_args () { let args = vec ! ["--stage" . into () , "1" . into () , "--version" . into () , "2" . into () , "--target" . into () , "x86_64-unknown-linux" . into () ,] ; assert_eq ! (parse_value_from_args (args . as_slice () , "--stage") . unwrap () , "1") ; assert_eq ! (parse_value_from_args (args . as_slice () , "--version") . unwrap () , "2") ; assert_eq ! (parse_value_from_args (args . as_slice () , "--target") . unwrap () , "x86_64-unknown-linux") ; assert ! (parse_value_from_args (args . as_slice () , "random-key") . is_none ()) ; let args = vec ! ["app-name" . into () , "--key" . into () , "value" . into () , "random-value" . into () , "--sysroot=/x/y/z" . into () ,] ; assert_eq ! (parse_value_from_args (args . as_slice () , "--key") . unwrap () , "value") ; assert_eq ! (parse_value_from_args (args . as_slice () , "--sysroot") . unwrap () , "/x/y/z") ; }
};
}
