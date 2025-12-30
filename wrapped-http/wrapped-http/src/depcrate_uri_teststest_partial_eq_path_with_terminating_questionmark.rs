// Generated macro for test_partial_eq_path_with_terminating_questionmark (function)
macro_rules! Depcrate_uri_teststest_partial_eq_path_with_terminating_questionmark {
() => {
// Module: crate::uri::tests
// Provides: {"test_partial_eq_path_with_terminating_questionmark"}
// Dependencies: {}
# [test] fn test_partial_eq_path_with_terminating_questionmark () { let a = "/path" ; let uri = Uri :: from_str ("/path?") . expect ("first parse") ; assert_eq ! (uri , a) ; }
};
}
