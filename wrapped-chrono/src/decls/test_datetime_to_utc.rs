macro_rules! deps {
    () => {
        FixedOffset!();
        DateTime!();
        Utc!();
    };
}

macro_rules! test_datetime_to_utc {
    () => {
        deps!();
        # [test] fn test_datetime_to_utc () { let dt = FixedOffset :: east_opt (3600) . unwrap () . with_ymd_and_hms (2020 , 2 , 22 , 23 , 24 , 25) . unwrap () ; let dt_utc : DateTime < Utc > = dt . to_utc () ; assert_eq ! (dt , dt_utc) ; }
    };
}

test_datetime_to_utc!()