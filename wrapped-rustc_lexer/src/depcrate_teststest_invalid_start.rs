// Generated macro for test_invalid_start (function)
macro_rules! Depcrate_teststest_invalid_start {
() => {
// Module: crate::tests
// Provides: {"test_invalid_start"}
// Dependencies: {}
# [test] fn test_invalid_start () { check_raw_str (r##"#~"abc"#"## , Err (RawStrError :: InvalidStarter { bad_char : '~' })) ; }
};
}
