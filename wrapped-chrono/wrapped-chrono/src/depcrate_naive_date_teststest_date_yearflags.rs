// Generated macro for test_date_yearflags (function)
macro_rules! Depcrate_naive_date_teststest_date_yearflags {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_yearflags"}
// Dependencies: {}
# [test] fn test_date_yearflags () { for (year , year_flags , _) in YEAR_FLAGS { assert_eq ! (NaiveDate :: from_yo_opt (year , 1) . unwrap () . year_flags () , year_flags) ; } }
};
}
