// Generated macro for test_local_beyond_max_datetime (function)
macro_rules! Depcrate_datetime_teststest_local_beyond_max_datetime {
() => {
// Module: crate::datetime::tests
// Provides: {"test_local_beyond_max_datetime"}
// Dependencies: {}
# [test] # [should_panic] fn test_local_beyond_max_datetime () { let max = FixedOffset :: east_opt (2 * 60 * 60) . unwrap () . from_utc_datetime (& NaiveDateTime :: MAX) ; let _ = max . naive_local () ; }
};
}
