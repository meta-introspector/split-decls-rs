macro_rules! deps {
    () => {
        FixedOffset!();
        Utc!();
        DateTime!();
    };
}

macro_rules! test_to_string_round_trip {
    () => {
        deps!();
        # [test] fn test_to_string_round_trip () { let dt = Utc . with_ymd_and_hms (2000 , 1 , 1 , 0 , 0 , 0) . unwrap () ; let _dt : DateTime < Utc > = dt . to_string () . parse () . unwrap () ; let ndt_fixed = dt . with_timezone (& FixedOffset :: east_opt (3600) . unwrap ()) ; let _dt : DateTime < FixedOffset > = ndt_fixed . to_string () . parse () . unwrap () ; let ndt_fixed = dt . with_timezone (& FixedOffset :: east_opt (0) . unwrap ()) ; let _dt : DateTime < FixedOffset > = ndt_fixed . to_string () . parse () . unwrap () ; }
    };
}

test_to_string_round_trip!();