macro_rules! deps {
    () => {
        DateTime!();
        Days!();
        Utc!();
        TimeDelta!();
    };
}

macro_rules! test_nanosecond_range {
    () => {
        deps!();
        # [test] fn test_nanosecond_range () { const A_BILLION : i64 = 1_000_000_000 ; let maximum = "2262-04-11T23:47:16.854775804UTC" ; let parsed : DateTime < Utc > = maximum . parse () . unwrap () ; let nanos = parsed . timestamp_nanos_opt () . unwrap () ; assert_eq ! (parsed , DateTime ::< Utc >:: from_timestamp (nanos / A_BILLION , (nanos % A_BILLION) as u32) . unwrap ()) ; let minimum = "1677-09-21T00:12:44.000000000UTC" ; let parsed : DateTime < Utc > = minimum . parse () . unwrap () ; let nanos = parsed . timestamp_nanos_opt () . unwrap () ; assert_eq ! (parsed , DateTime ::< Utc >:: from_timestamp (nanos / A_BILLION , (nanos % A_BILLION) as u32) . unwrap ()) ; let maximum = "2262-04-11T23:47:16.854775804UTC" ; let parsed : DateTime < Utc > = maximum . parse () . unwrap () ; let beyond_max = parsed + TimeDelta :: try_milliseconds (300) . unwrap () ; assert ! (beyond_max . timestamp_nanos_opt () . is_none ()) ; let maximum = "2262-04-11T23:47:16.854775804UTC" ; let parsed : DateTime < Utc > = maximum . parse () . unwrap () ; let beyond_max = parsed + Days :: new (365) ; assert ! (beyond_max . timestamp_nanos_opt () . is_none ()) ; }
    };
}

test_nanosecond_range!();