// Generated macro for test_into_error (function)
macro_rules! Depcrate_teststest_into_error {
() => {
// Module: crate::tests
// Provides: {"test_into_error"}
// Dependencies: {}
# [test] fn test_into_error () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let short_error_ss = Box :: < dyn std :: error :: Error + Send + Sync > :: from (CompactString :: new (short)) ; assert_eq ! (short , format ! ("{short_error_ss}")) ; assert_eq ! (format ! ("{short:?}") , format ! ("{short_error_ss:?}")) ; let long_error_ss = Box :: < dyn std :: error :: Error + Send + Sync > :: from (CompactString :: new (long)) ; assert_eq ! (long , format ! ("{long_error_ss}")) ; assert_eq ! (format ! ("{long:?}") , format ! ("{long_error_ss:?}")) ; let short_error = Box :: < dyn std :: error :: Error > :: from (CompactString :: new (short)) ; assert_eq ! (short , format ! ("{short_error}")) ; assert_eq ! (format ! ("{short:?}") , format ! ("{short_error:?}")) ; let long_error = Box :: < dyn std :: error :: Error > :: from (CompactString :: new (long)) ; assert_eq ! (long , format ! ("{long_error}")) ; assert_eq ! (format ! ("{long:?}") , format ! ("{long_error:?}")) ; }
};
}
