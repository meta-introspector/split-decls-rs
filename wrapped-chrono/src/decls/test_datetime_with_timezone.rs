macro_rules! deps {
    () => {
        Local!();
        Utc!();
    };
}

macro_rules! test_datetime_with_timezone {
    () => {
        deps!();
        # [test] # [cfg (feature = "clock")] fn test_datetime_with_timezone () { let local_now = Local :: now () ; let utc_now = local_now . with_timezone (& Utc) ; let local_now2 = utc_now . with_timezone (& Local) ; assert_eq ! (local_now , local_now2) ; }
    };
}

test_datetime_with_timezone!();