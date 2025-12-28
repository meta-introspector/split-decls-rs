macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveDate!();
    };
}

macro_rules! test_datetime_add_sub_invariant {
    () => {
        deps!();
        # [test] fn test_datetime_add_sub_invariant () { let base = NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap () . and_hms_opt (0 , 0 , 0) . unwrap () ; let t = - 946684799990000 ; let time = base + TimeDelta :: microseconds (t) ; assert_eq ! (t , time . signed_duration_since (base) . num_microseconds () . unwrap ()) ; }
    };
}

test_datetime_add_sub_invariant!()