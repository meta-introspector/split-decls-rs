macro_rules! deps {
    () => {
        Utc!();
        SecondsFormat!();
    };
}

macro_rules! test_rfc3339_opts_nonexhaustive {
    () => {
        deps!();
        # [test] # [should_panic] # [cfg (feature = "alloc")] fn test_rfc3339_opts_nonexhaustive () { use crate :: SecondsFormat ; let dt = Utc . with_ymd_and_hms (1999 , 10 , 9 , 1 , 2 , 3) . unwrap () ; let _ = dt . to_rfc3339_opts (SecondsFormat :: __NonExhaustive , true) ; }
    };
}

test_rfc3339_opts_nonexhaustive!();