// Generated macro for test_isoweekdate_with_yearflags (function)
macro_rules! Depcrate_naive_date_teststest_isoweekdate_with_yearflags {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_isoweekdate_with_yearflags"}
// Dependencies: {}
# [test] fn test_isoweekdate_with_yearflags () { for (year , year_flags , _) in YEAR_FLAGS { let jan4 = NaiveDate :: from_ymd_opt (year , 1 , 4) . unwrap () ; let iso_week = jan4 . iso_week () ; assert_eq ! (jan4 . year_flags () , year_flags) ; assert_eq ! (iso_week . week () , 1) ; } }
};
}
