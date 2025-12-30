// Generated macro for test_day_iterator_limit (function)
macro_rules! Depcrate_naive_date_teststest_day_iterator_limit {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_day_iterator_limit"}
// Dependencies: {}
# [test] fn test_day_iterator_limit () { assert_eq ! (NaiveDate :: from_ymd_opt (MAX_YEAR , 12 , 29) . unwrap () . iter_days () . take (4) . count () , 2) ; assert_eq ! (NaiveDate :: from_ymd_opt (MIN_YEAR , 1 , 3) . unwrap () . iter_days () . rev () . take (4) . count () , 2) ; }
};
}
