macro_rules! deps {
    () => {
        NaiveDate!();
        Duration!();
    };
}

macro_rules! test_core_duration_max {
    () => {
        deps!();
        # [test] # [should_panic] fn test_core_duration_max () { use core :: time :: Duration ; let mut utc_dt = NaiveDate :: from_ymd_opt (2023 , 8 , 29) . unwrap () . and_hms_opt (11 , 34 , 12) . unwrap () ; utc_dt += Duration :: MAX ; }
    };
}

test_core_duration_max!();