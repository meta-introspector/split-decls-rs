macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
    };
}

macro_rules! test_datetime_subassignment {
    () => {
        deps!();
        # [test] fn test_datetime_subassignment () { let ymdhms = | y , m , d , h , n , s | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () ; let mut date = ymdhms (2016 , 10 , 1 , 10 , 10 , 10) ; date -= TimeDelta :: try_minutes (10_000_000) . unwrap () ; assert_eq ! (date , ymdhms (1997 , 9 , 26 , 23 , 30 , 10)) ; date -= TimeDelta :: try_days (10) . unwrap () ; assert_eq ! (date , ymdhms (1997 , 9 , 16 , 23 , 30 , 10)) ; }
    };
}

test_datetime_subassignment!();