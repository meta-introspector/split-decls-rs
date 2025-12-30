// Generated macro for test_date_to_epoch_days (function)
macro_rules! Depcrate_naive_date_teststest_date_to_epoch_days {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_to_epoch_days"}
// Dependencies: {}
# [test] fn test_date_to_epoch_days () { assert_eq ! (NaiveDate :: from_ymd_opt (1970 , 1 , 1) . unwrap () . to_epoch_days () , 0) ; for year in - 9999 .. 10001 { assert_eq ! (NaiveDate :: from_ymd_opt (year , 1 , 1) . unwrap () . to_epoch_days () , NaiveDate :: from_ymd_opt (year - 1 , 12 , 31) . unwrap () . to_epoch_days () + 1) ; } }
};
}
