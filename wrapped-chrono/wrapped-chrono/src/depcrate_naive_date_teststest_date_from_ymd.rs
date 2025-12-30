// Generated macro for test_date_from_ymd (function)
macro_rules! Depcrate_naive_date_teststest_date_from_ymd {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_from_ymd"}
// Dependencies: {}
# [test] fn test_date_from_ymd () { let from_ymd = NaiveDate :: from_ymd_opt ; assert ! (from_ymd (2012 , 0 , 1) . is_none ()) ; assert ! (from_ymd (2012 , 1 , 1) . is_some ()) ; assert ! (from_ymd (2012 , 2 , 29) . is_some ()) ; assert ! (from_ymd (2014 , 2 , 29) . is_none ()) ; assert ! (from_ymd (2014 , 3 , 0) . is_none ()) ; assert ! (from_ymd (2014 , 3 , 1) . is_some ()) ; assert ! (from_ymd (2014 , 3 , 31) . is_some ()) ; assert ! (from_ymd (2014 , 3 , 32) . is_none ()) ; assert ! (from_ymd (2014 , 12 , 31) . is_some ()) ; assert ! (from_ymd (2014 , 13 , 1) . is_none ()) ; }
};
}
