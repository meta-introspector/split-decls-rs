macro_rules! deps {
    () => {
        DateTime!();
        NaiveDate!();
    };
}

macro_rules! test_datetime_from_timestamp {
    () => {
        deps!();
        # [test] fn test_datetime_from_timestamp () { let ymdhms = | y , m , d , h , n , s | { NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () . and_utc () } ; assert_eq ! (DateTime :: from_timestamp_secs (- 1) , Some (ymdhms (1969 , 12 , 31 , 23 , 59 , 59))) ; assert_eq ! (DateTime :: from_timestamp_secs (0) , Some (ymdhms (1970 , 1 , 1 , 0 , 0 , 0))) ; assert_eq ! (DateTime :: from_timestamp_secs (1) , Some (ymdhms (1970 , 1 , 1 , 0 , 0 , 1))) ; assert_eq ! (DateTime :: from_timestamp_secs (1_000_000_000) , Some (ymdhms (2001 , 9 , 9 , 1 , 46 , 40))) ; assert_eq ! (DateTime :: from_timestamp_secs (0x7fffffff) , Some (ymdhms (2038 , 1 , 19 , 3 , 14 , 7))) ; assert_eq ! (DateTime :: from_timestamp_secs (i64 :: MIN) , None) ; assert_eq ! (DateTime :: from_timestamp_secs (i64 :: MAX) , None) ; }
    };
}

test_datetime_from_timestamp!();