macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_datetime_timestamp {
    () => {
        deps!();
        # [test] fn test_datetime_timestamp () { let to_timestamp = | y , m , d , h , n , s | { NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () . and_utc () . timestamp () } ; assert_eq ! (to_timestamp (1969 , 12 , 31 , 23 , 59 , 59) , - 1) ; assert_eq ! (to_timestamp (1970 , 1 , 1 , 0 , 0 , 0) , 0) ; assert_eq ! (to_timestamp (1970 , 1 , 1 , 0 , 0 , 1) , 1) ; assert_eq ! (to_timestamp (2001 , 9 , 9 , 1 , 46 , 40) , 1_000_000_000) ; assert_eq ! (to_timestamp (2038 , 1 , 19 , 3 , 14 , 7) , 0x7fffffff) ; }
    };
}

test_datetime_timestamp!()