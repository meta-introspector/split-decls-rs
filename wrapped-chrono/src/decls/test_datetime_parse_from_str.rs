macro_rules! deps {
    () => {
        NaiveDate!();
        NaiveDateTime!();
    };
}

macro_rules! test_datetime_parse_from_str {
    () => {
        deps!();
        # [test] fn test_datetime_parse_from_str () { let ymdhms = | y , m , d , h , n , s | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () ; let ymdhmsn = | y , m , d , h , n , s , nano | { NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_nano_opt (h , n , s , nano) . unwrap () } ; assert_eq ! (NaiveDateTime :: parse_from_str ("2014-5-7T12:34:56+09:30" , "%Y-%m-%dT%H:%M:%S%z") , Ok (ymdhms (2014 , 5 , 7 , 12 , 34 , 56))) ; assert_eq ! (NaiveDateTime :: parse_from_str ("2015-W06-1 000000" , "%G-W%V-%u%H%M%S") , Ok (ymdhms (2015 , 2 , 2 , 0 , 0 , 0))) ; assert_eq ! (NaiveDateTime :: parse_from_str ("Fri, 09 Aug 2013 23:54:35 GMT" , "%a, %d %b %Y %H:%M:%S GMT") , Ok (ymdhms (2013 , 8 , 9 , 23 , 54 , 35))) ; assert ! (NaiveDateTime :: parse_from_str ("Sat, 09 Aug 2013 23:54:35 GMT" , "%a, %d %b %Y %H:%M:%S GMT") . is_err ()) ; assert ! (NaiveDateTime :: parse_from_str ("2014-5-7 Q2 12:3456" , "%Y-%m-%d Q%q %H:%M:%S") . is_err ()) ; assert ! (NaiveDateTime :: parse_from_str ("12:34:56" , "%H:%M:%S") . is_err ()) ; assert_eq ! (NaiveDateTime :: parse_from_str ("1441497364" , "%s") , Ok (ymdhms (2015 , 9 , 5 , 23 , 56 , 4))) ; assert_eq ! (NaiveDateTime :: parse_from_str ("1283929614.1234" , "%s.%f") , Ok (ymdhmsn (2010 , 9 , 8 , 7 , 6 , 54 , 1234))) ; assert_eq ! (NaiveDateTime :: parse_from_str ("1441497364.649" , "%s%.3f") , Ok (ymdhmsn (2015 , 9 , 5 , 23 , 56 , 4 , 649000000))) ; assert_eq ! (NaiveDateTime :: parse_from_str ("1497854303.087654" , "%s%.6f") , Ok (ymdhmsn (2017 , 6 , 19 , 6 , 38 , 23 , 87654000))) ; assert_eq ! (NaiveDateTime :: parse_from_str ("1437742189.918273645" , "%s%.9f") , Ok (ymdhmsn (2015 , 7 , 24 , 12 , 49 , 49 , 918273645))) ; }
    };
}

test_datetime_parse_from_str!();