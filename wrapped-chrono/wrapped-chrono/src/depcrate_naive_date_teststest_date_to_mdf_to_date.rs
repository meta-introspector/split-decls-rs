// Generated macro for test_date_to_mdf_to_date (function)
macro_rules! Depcrate_naive_date_teststest_date_to_mdf_to_date {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_to_mdf_to_date"}
// Dependencies: {}
# [test] fn test_date_to_mdf_to_date () { for (year , year_flags , _) in YEAR_FLAGS { for ordinal in 1 ..= year_flags . ndays () { let date = NaiveDate :: from_yo_opt (year , ordinal) . unwrap () ; assert_eq ! (date , NaiveDate :: from_mdf (date . year () , date . mdf ()) . unwrap ()) ; } } }
};
}
