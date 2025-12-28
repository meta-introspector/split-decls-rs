macro_rules! deps {
    () => {
        Utc!();
        Local!();
    };
}

macro_rules! test_datetime_format_with_local {
    () => {
        deps!();
        # [test] # [cfg (feature = "clock")] fn test_datetime_format_with_local () { let dt = Local :: now () . with_month (5) . unwrap () ; assert_eq ! (dt . format ("%Y") . to_string () , dt . with_timezone (& Utc) . format ("%Y") . to_string ()) ; }
    };
}

test_datetime_format_with_local!()