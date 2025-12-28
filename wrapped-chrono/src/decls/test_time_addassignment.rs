macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveTime!();
    };
}

macro_rules! test_time_addassignment {
    () => {
        deps!();
        # [test] fn test_time_addassignment () { let hms = | h , m , s | NaiveTime :: from_hms_opt (h , m , s) . unwrap () ; let mut time = hms (12 , 12 , 12) ; time += TimeDelta :: try_hours (10) . unwrap () ; assert_eq ! (time , hms (22 , 12 , 12)) ; time += TimeDelta :: try_hours (10) . unwrap () ; assert_eq ! (time , hms (8 , 12 , 12)) ; }
    };
}

test_time_addassignment!();