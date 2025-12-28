macro_rules! deps {
    () => {
        Utc!();
        NaiveDate!();
    };
}

macro_rules! test_and_utc {
    () => {
        deps!();
        # [test] fn test_and_utc () { let ndt = NaiveDate :: from_ymd_opt (2023 , 1 , 30) . unwrap () . and_hms_opt (19 , 32 , 33) . unwrap () ; let dt_utc = ndt . and_utc () ; assert_eq ! (dt_utc . naive_local () , ndt) ; assert_eq ! (dt_utc . timezone () , Utc) ; }
    };
}

test_and_utc!();