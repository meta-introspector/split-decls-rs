macro_rules! deps {
    () => {
        NaiveDateTime!();
        FixedOffset!();
    };
}

macro_rules! test_local_beyond_min_datetime {
    () => {
        deps!();
        # [test] # [should_panic] fn test_local_beyond_min_datetime () { let min = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () . from_utc_datetime (& NaiveDateTime :: MIN) ; let _ = min . naive_local () ; }
    };
}

test_local_beyond_min_datetime!();