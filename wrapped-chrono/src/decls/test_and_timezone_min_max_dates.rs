macro_rules! deps {
    () => {
        NaiveDateTime!();
        MappedLocalTime!();
        FixedOffset!();
    };
}

macro_rules! test_and_timezone_min_max_dates {
    () => {
        deps!();
        # [test] fn test_and_timezone_min_max_dates () { for offset_hour in - 23 ..= 23 { dbg ! (offset_hour) ; let offset = FixedOffset :: east_opt (offset_hour * 60 * 60) . unwrap () ; let local_max = NaiveDateTime :: MAX . and_local_timezone (offset) ; if offset_hour >= 0 { assert_eq ! (local_max . unwrap () . naive_local () , NaiveDateTime :: MAX) ; } else { assert_eq ! (local_max , MappedLocalTime :: None) ; } let local_min = NaiveDateTime :: MIN . and_local_timezone (offset) ; if offset_hour <= 0 { assert_eq ! (local_min . unwrap () . naive_local () , NaiveDateTime :: MIN) ; } else { assert_eq ! (local_min , MappedLocalTime :: None) ; } } }
    };
}

test_and_timezone_min_max_dates!()