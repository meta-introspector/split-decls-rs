// Generated macro for test_should_run (function)
macro_rules! Depcrate_internal_test_filtertest_should_run {
() => {
// Module: crate::internal::test_filter
// Provides: {"test_should_run"}
// Dependencies: {}
pub fn test_should_run (test_name : & str) -> bool { let test_filter = TEST_FILTER . get_or_init (| | { if let Ok (testbridge_test_only) = std :: env :: var ("TESTBRIDGE_TEST_ONLY") { Box :: new (get_test_filter (& testbridge_test_only)) } else { Box :: new (AcceptAll) } }) ; test_filter . filter (test_name) }
};
}
