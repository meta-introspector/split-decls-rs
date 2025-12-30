// Generated macro for test_local_beyond_min_datetime (function)
macro_rules! Depcrate_datetime_teststest_local_beyond_min_datetime {
() => {
// Module: crate::datetime::tests
// Provides: {"test_local_beyond_min_datetime"}
// Dependencies: {}
# [test] # [should_panic] fn test_local_beyond_min_datetime () { let min = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () . from_utc_datetime (& NaiveDateTime :: MIN) ; let _ = min . naive_local () ; }
};
}
