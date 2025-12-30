// Generated macro for test_datetime_with_timezone (function)
macro_rules! Depcrate_datetime_teststest_datetime_with_timezone {
() => {
// Module: crate::datetime::tests
// Provides: {"test_datetime_with_timezone"}
// Dependencies: {}
# [test] # [cfg (feature = "clock")] fn test_datetime_with_timezone () { let local_now = Local :: now () ; let utc_now = local_now . with_timezone (& Utc) ; let local_now2 = utc_now . with_timezone (& Local) ; assert_eq ! (local_now , local_now2) ; }
};
}
