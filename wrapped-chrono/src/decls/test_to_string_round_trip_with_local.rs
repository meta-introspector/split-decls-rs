macro_rules! deps {
    () => {
        Local!();
        DateTime!();
        FixedOffset!();
    };
}

macro_rules! test_to_string_round_trip_with_local {
    () => {
        deps!();
        # [test] # [cfg (feature = "clock")] fn test_to_string_round_trip_with_local () { let ndt = Local :: now () ; let _dt : DateTime < FixedOffset > = ndt . to_string () . parse () . unwrap () ; }
    };
}

test_to_string_round_trip_with_local!()