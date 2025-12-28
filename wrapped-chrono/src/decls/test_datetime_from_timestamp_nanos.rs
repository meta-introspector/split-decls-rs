macro_rules! deps {
    () => {
        DateTime!();
        Utc!();
    };
}

macro_rules! test_datetime_from_timestamp_nanos {
    () => {
        deps!();
        # [test] fn test_datetime_from_timestamp_nanos () { let valid_map = [(1662921288000000000 , "2022-09-11 18:34:48.000000000") , (1662921288123456000 , "2022-09-11 18:34:48.123456000") , (1662921288123456789 , "2022-09-11 18:34:48.123456789") , (1662921287890000000 , "2022-09-11 18:34:47.890000000") , (- 2208936075000000000 , "1900-01-01 14:38:45.000000000") , (- 5337182663000000000 , "1800-11-15 01:15:37.000000000") , (0 , "1970-01-01 00:00:00.000000000") , (119731017000000000 , "1973-10-17 18:36:57.000000000") , (1234567890000000000 , "2009-02-13 23:31:30.000000000") , (2034061609000000000 , "2034-06-16 09:06:49.000000000") ,] ; for (timestamp_nanos , _formatted) in valid_map . iter () . copied () { let datetime = DateTime :: from_timestamp_nanos (timestamp_nanos) ; assert_eq ! (timestamp_nanos , datetime . timestamp_nanos_opt () . unwrap ()) ; # [cfg (feature = "alloc")] assert_eq ! (datetime . format ("%F %T%.9f") . to_string () , _formatted) ; } const A_BILLION : i64 = 1_000_000_000 ; let maximum = "2262-04-11T23:47:16.854775804UTC" ; let parsed : DateTime < Utc > = maximum . parse () . unwrap () ; let nanos = parsed . timestamp_nanos_opt () . unwrap () ; assert_eq ! (Some (DateTime :: from_timestamp_nanos (nanos)) , DateTime :: from_timestamp (nanos / A_BILLION , (nanos % A_BILLION) as u32)) ; let minimum = "1677-09-21T00:12:44.000000000UTC" ; let parsed : DateTime < Utc > = minimum . parse () . unwrap () ; let nanos = parsed . timestamp_nanos_opt () . unwrap () ; assert_eq ! (Some (DateTime :: from_timestamp_nanos (nanos)) , DateTime :: from_timestamp (nanos / A_BILLION , (nanos % A_BILLION) as u32)) ; let secs_test = [0 , 1 , 2 , 1000 , 1234 , 12345678 , - 1 , - 2 , - 1000 , - 12345678] ; for secs in secs_test . iter () . copied () { assert_eq ! (Some (DateTime :: from_timestamp_nanos (secs * 1_000_000_000)) , DateTime :: from_timestamp_secs (secs)) ; } }
    };
}

test_datetime_from_timestamp_nanos!()