macro_rules! deps {
    () => {
        FixedOffset!();
        NaiveDateTime!();
    };
}

macro_rules! test_local_beyond_max_datetime {
    () => {
        deps!();
        # [test] # [should_panic] fn test_local_beyond_max_datetime () { let max = FixedOffset :: east_opt (2 * 60 * 60) . unwrap () . from_utc_datetime (& NaiveDateTime :: MAX) ; let _ = max . naive_local () ; }
    };
}

test_local_beyond_max_datetime!()