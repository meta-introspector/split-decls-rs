// Generated macro for test_week_iterator_limit (function)
macro_rules! Depcrate_naive_date_teststest_week_iterator_limit {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_week_iterator_limit"}
// Dependencies: {}
# [test] fn test_week_iterator_limit () { assert_eq ! (NaiveDate :: from_ymd_opt (MAX_YEAR , 12 , 12) . unwrap () . iter_weeks () . take (4) . count () , 2) ; assert_eq ! (NaiveDate :: from_ymd_opt (MIN_YEAR , 1 , 15) . unwrap () . iter_weeks () . rev () . take (4) . count () , 2) ; }
};
}
