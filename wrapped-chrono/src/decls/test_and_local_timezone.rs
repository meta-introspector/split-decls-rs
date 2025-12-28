macro_rules! deps {
    () => {
        Utc!();
        NaiveDate!();
        FixedOffset!();
    };
}

macro_rules! test_and_local_timezone {
    () => {
        deps!();
        # [test] fn test_and_local_timezone () { let ndt = NaiveDate :: from_ymd_opt (2022 , 6 , 15) . unwrap () . and_hms_opt (18 , 59 , 36) . unwrap () ; let dt_utc = ndt . and_utc () ; assert_eq ! (dt_utc . naive_local () , ndt) ; assert_eq ! (dt_utc . timezone () , Utc) ; let offset_tz = FixedOffset :: west_opt (4 * 3600) . unwrap () ; let dt_offset = ndt . and_local_timezone (offset_tz) . unwrap () ; assert_eq ! (dt_offset . naive_local () , ndt) ; assert_eq ! (dt_offset . timezone () , offset_tz) ; }
    };
}

test_and_local_timezone!();