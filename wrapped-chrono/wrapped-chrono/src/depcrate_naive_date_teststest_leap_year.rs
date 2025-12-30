// Generated macro for test_leap_year (function)
macro_rules! Depcrate_naive_date_teststest_leap_year {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_leap_year"}
// Dependencies: {}
# [test] fn test_leap_year () { for year in 0 ..= MAX_YEAR { let date = NaiveDate :: from_ymd_opt (year , 1 , 1) . unwrap () ; let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) ; assert_eq ! (date . leap_year () , is_leap) ; assert_eq ! (date . leap_year () , date . with_ordinal (366) . is_some ()) ; } }
};
}
