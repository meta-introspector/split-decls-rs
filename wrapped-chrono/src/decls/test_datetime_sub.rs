macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveDateTime!();
        NaiveDate!();
    };
}

macro_rules! test_datetime_sub {
    () => {
        deps!();
        # [test] fn test_datetime_sub () { let ymdhms = | y , m , d , h , n , s | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () ; let since = NaiveDateTime :: signed_duration_since ; assert_eq ! (since (ymdhms (2014 , 5 , 6 , 7 , 8 , 9) , ymdhms (2014 , 5 , 6 , 7 , 8 , 9)) , TimeDelta :: zero ()) ; assert_eq ! (since (ymdhms (2014 , 5 , 6 , 7 , 8 , 10) , ymdhms (2014 , 5 , 6 , 7 , 8 , 9)) , TimeDelta :: try_seconds (1) . unwrap ()) ; assert_eq ! (since (ymdhms (2014 , 5 , 6 , 7 , 8 , 9) , ymdhms (2014 , 5 , 6 , 7 , 8 , 10)) , TimeDelta :: try_seconds (- 1) . unwrap ()) ; assert_eq ! (since (ymdhms (2014 , 5 , 7 , 7 , 8 , 9) , ymdhms (2014 , 5 , 6 , 7 , 8 , 10)) , TimeDelta :: try_seconds (86399) . unwrap ()) ; assert_eq ! (since (ymdhms (2001 , 9 , 9 , 1 , 46 , 39) , ymdhms (1970 , 1 , 1 , 0 , 0 , 0)) , TimeDelta :: try_seconds (999_999_999) . unwrap ()) ; }
    };
}

test_datetime_sub!();