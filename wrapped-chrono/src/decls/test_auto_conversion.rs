macro_rules! deps {
    () => {
        DateTime!();
        Utc!();
        FixedOffset!();
    };
}

macro_rules! test_auto_conversion {
    () => {
        deps!();
        # [test] fn test_auto_conversion () { let utc_dt = Utc . with_ymd_and_hms (2018 , 9 , 5 , 23 , 58 , 0) . unwrap () ; let cdt_dt = FixedOffset :: west_opt (5 * 60 * 60) . unwrap () . with_ymd_and_hms (2018 , 9 , 5 , 18 , 58 , 0) . unwrap () ; let utc_dt2 : DateTime < Utc > = cdt_dt . into () ; assert_eq ! (utc_dt , utc_dt2) ; }
    };
}

test_auto_conversion!();