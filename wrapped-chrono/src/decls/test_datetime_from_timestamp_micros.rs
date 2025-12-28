macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! test_datetime_from_timestamp_micros {
    () => {
        deps!();
        # [test] fn test_datetime_from_timestamp_micros () { let valid_map = [(1662921288000000 , "2022-09-11 18:34:48.000000000") , (1662921288123456 , "2022-09-11 18:34:48.123456000") , (1662921287890000 , "2022-09-11 18:34:47.890000000") , (- 2208936075000000 , "1900-01-01 14:38:45.000000000") , (0 , "1970-01-01 00:00:00.000000000") , (119731017000000 , "1973-10-17 18:36:57.000000000") , (1234567890000000 , "2009-02-13 23:31:30.000000000") , (2034061609000000 , "2034-06-16 09:06:49.000000000") ,] ; for (timestamp_micros , _formatted) in valid_map . iter () . copied () { let datetime = DateTime :: from_timestamp_micros (timestamp_micros) . unwrap () ; assert_eq ! (timestamp_micros , datetime . timestamp_micros ()) ; # [cfg (feature = "alloc")] assert_eq ! (datetime . format ("%F %T%.9f") . to_string () , _formatted) ; } let invalid = [i64 :: MAX , i64 :: MIN] ; for timestamp_micros in invalid . iter () . copied () { let datetime = DateTime :: from_timestamp_micros (timestamp_micros) ; assert ! (datetime . is_none ()) ; } let secs_test = [0 , 1 , 2 , 1000 , 1234 , 12345678 , - 1 , - 2 , - 1000 , - 12345678] ; for secs in secs_test . iter () . copied () { assert_eq ! (DateTime :: from_timestamp_micros (secs * 1_000_000) , DateTime :: from_timestamp_secs (secs)) ; } }
    };
}

test_datetime_from_timestamp_micros!()