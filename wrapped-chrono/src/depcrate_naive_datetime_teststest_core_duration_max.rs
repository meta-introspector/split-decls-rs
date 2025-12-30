// Generated macro for test_core_duration_max (function)
macro_rules! Depcrate_naive_datetime_teststest_core_duration_max {
() => {
// Module: crate::naive::datetime::tests
// Provides: {"test_core_duration_max"}
// Dependencies: {}
# [test] # [should_panic] fn test_core_duration_max () { use core :: time :: Duration ; let mut utc_dt = NaiveDate :: from_ymd_opt (2023 , 8 , 29) . unwrap () . and_hms_opt (11 , 34 , 12) . unwrap () ; utc_dt += Duration :: MAX ; }
};
}
