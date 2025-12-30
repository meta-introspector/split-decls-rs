// Generated macro for test_with_0_overflow (function)
macro_rules! Depcrate_naive_date_teststest_with_0_overflow {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_with_0_overflow"}
// Dependencies: {}
# [test] fn test_with_0_overflow () { let dt = NaiveDate :: from_ymd_opt (2023 , 4 , 18) . unwrap () ; assert ! (dt . with_month0 (4294967295) . is_none ()) ; assert ! (dt . with_day0 (4294967295) . is_none ()) ; assert ! (dt . with_ordinal0 (4294967295) . is_none ()) ; }
};
}
