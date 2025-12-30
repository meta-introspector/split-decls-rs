// Generated macro for test_constant_time (function)
macro_rules! Depcrate_tests_c_constant_time_teststest_constant_time {
() => {
// Module: crate::tests::c_constant_time_tests
// Provides: {"test_constant_time"}
// Dependencies: {}
# [test] fn test_constant_time () -> Result < () , error :: Unspecified > { prefixed_extern ! { fn bssl_constant_time_test_main () -> bssl :: Result ; } Result :: from (unsafe { bssl_constant_time_test_main () }) }
};
}
