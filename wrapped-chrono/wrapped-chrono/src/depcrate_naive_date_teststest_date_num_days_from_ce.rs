// Generated macro for test_date_num_days_from_ce (function)
macro_rules! Depcrate_naive_date_teststest_date_num_days_from_ce {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_num_days_from_ce"}
// Dependencies: {}
# [test] fn test_date_num_days_from_ce () { assert_eq ! (NaiveDate :: from_ymd_opt (1 , 1 , 1) . unwrap () . num_days_from_ce () , 1) ; for year in - 9999 .. 10001 { assert_eq ! (NaiveDate :: from_ymd_opt (year , 1 , 1) . unwrap () . num_days_from_ce () , NaiveDate :: from_ymd_opt (year - 1 , 12 , 31) . unwrap () . num_days_from_ce () + 1) ; } }
};
}
