// Generated macro for test_date_from_str (function)
macro_rules! Depcrate_naive_date_teststest_date_from_str {
() => {
// Module: crate::naive::date::tests
// Provides: {"test_date_from_str"}
// Dependencies: {}
# [test] fn test_date_from_str () { let valid = ["-0000000123456-1-2" , "    -123456 - 1 - 2    " , "-12345-1-2" , "-1234-12-31" , "-7-6-5" , "350-2-28" , "360-02-29" , "0360-02-29" , "2015-2 -18" , "2015-02-18" , "+70-2-18" , "+70000-2-18" , "+00007-2-18" ,] ; for & s in & valid { eprintln ! ("test_date_from_str valid {s:?}") ; let d = match s . parse :: < NaiveDate > () { Ok (d) => d , Err (e) => panic ! ("parsing `{s}` has failed: {e}") , } ; eprintln ! ("d {d:?} (NaiveDate)") ; let s_ = format ! ("{d:?}") ; eprintln ! ("s_ {s_:?}") ; let d_ = match s_ . parse :: < NaiveDate > () { Ok (d) => d , Err (e) => { panic ! ("`{s}` is parsed into `{d:?}`, but reparsing that has failed: {e}") } } ; eprintln ! ("d_ {d_:?} (NaiveDate)") ; assert ! (d == d_ , "`{s}` is parsed into `{d:?}`, but reparsed result \
                            `{d_:?}` does not match") ; } let invalid = ["" , "x" , "Fri, 09 Aug 2013 GMT" , "Sat Jun 30 2012" , "1441497364.649" , "+1441497364.649" , "+1441497364" , "2014/02/03" , "2014" , "2014-01" , "2014-01-00" , "2014-11-32" , "2014-13-01" , "2014-13-57" , "9999999-9-9" ,] ; for & s in & invalid { eprintln ! ("test_date_from_str invalid {s:?}") ; assert ! (s . parse ::< NaiveDate > () . is_err ()) ; } }
};
}
