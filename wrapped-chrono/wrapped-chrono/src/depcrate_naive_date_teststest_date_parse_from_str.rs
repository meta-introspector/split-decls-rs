// Generated macro for test_date_parse_from_str (function)
macro_rules! Depcrate_naive_date_teststest_date_parse_from_str {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_parse_from_str"}
// Dependencies: {}
# [test] fn test_date_parse_from_str () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; assert_eq ! (NaiveDate :: parse_from_str ("2014-5-7T12:34:56+09:30" , "%Y-%m-%dT%H:%M:%S%z") , Ok (ymd (2014 , 5 , 7))) ; assert_eq ! (NaiveDate :: parse_from_str ("2015-W06-1=2015-033 Q1" , "%G-W%V-%u = %Y-%j Q%q") , Ok (ymd (2015 , 2 , 2))) ; assert_eq ! (NaiveDate :: parse_from_str ("Fri, 09 Aug 13" , "%a, %d %b %y") , Ok (ymd (2013 , 8 , 9))) ; assert ! (NaiveDate :: parse_from_str ("Sat, 09 Aug 2013" , "%a, %d %b %Y") . is_err ()) ; assert ! (NaiveDate :: parse_from_str ("2014-57" , "%Y-%m-%d") . is_err ()) ; assert ! (NaiveDate :: parse_from_str ("2014" , "%Y") . is_err ()) ; assert ! (NaiveDate :: parse_from_str ("2014-5-7 Q3" , "%Y-%m-%d Q%q") . is_err ()) ; assert_eq ! (NaiveDate :: parse_from_str ("2020-01-0" , "%Y-%W-%w") . ok () , NaiveDate :: from_ymd_opt (2020 , 1 , 12) ,) ; assert_eq ! (NaiveDate :: parse_from_str ("2019-01-0" , "%Y-%W-%w") . ok () , NaiveDate :: from_ymd_opt (2019 , 1 , 13) ,) ; }
};
}
