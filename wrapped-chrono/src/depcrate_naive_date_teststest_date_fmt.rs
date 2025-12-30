// Generated macro for test_date_fmt (function)
macro_rules! Depcrate_naive_date_teststest_date_fmt {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_fmt"}
// Dependencies: {}
# [test] fn test_date_fmt () { assert_eq ! (format ! ("{:?}" , NaiveDate :: from_ymd_opt (2012 , 3 , 4) . unwrap ()) , "2012-03-04") ; assert_eq ! (format ! ("{:?}" , NaiveDate :: from_ymd_opt (0 , 3 , 4) . unwrap ()) , "0000-03-04") ; assert_eq ! (format ! ("{:?}" , NaiveDate :: from_ymd_opt (- 307 , 3 , 4) . unwrap ()) , "-0307-03-04") ; assert_eq ! (format ! ("{:?}" , NaiveDate :: from_ymd_opt (12345 , 3 , 4) . unwrap ()) , "+12345-03-04") ; assert_eq ! (NaiveDate :: from_ymd_opt (2012 , 3 , 4) . unwrap () . to_string () , "2012-03-04") ; assert_eq ! (NaiveDate :: from_ymd_opt (0 , 3 , 4) . unwrap () . to_string () , "0000-03-04") ; assert_eq ! (NaiveDate :: from_ymd_opt (- 307 , 3 , 4) . unwrap () . to_string () , "-0307-03-04") ; assert_eq ! (NaiveDate :: from_ymd_opt (12345 , 3 , 4) . unwrap () . to_string () , "+12345-03-04") ; assert_eq ! (format ! ("{:+30?}" , NaiveDate :: from_ymd_opt (1234 , 5 , 6) . unwrap ()) , "1234-05-06") ; assert_eq ! (format ! ("{:30?}" , NaiveDate :: from_ymd_opt (12345 , 6 , 7) . unwrap ()) , "+12345-06-07") ; }
};
}
