macro_rules! deps {
    () => {
        FixedOffset!();
        Days!();
    };
}

macro_rules! test_datetime_sub_days {
    () => {
        deps!();
        # [test] fn test_datetime_sub_days () { let est = FixedOffset :: west_opt (5 * 60 * 60) . unwrap () ; let kst = FixedOffset :: east_opt (9 * 60 * 60) . unwrap () ; assert_eq ! (format ! ("{}" , est . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () - Days :: new (5)) , "2014-05-01 07:08:09 -05:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () - Days :: new (5)) , "2014-05-01 07:08:09 +09:00") ; assert_eq ! (format ! ("{}" , est . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () - Days :: new (35)) , "2014-04-01 07:08:09 -05:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () - Days :: new (35)) , "2014-04-01 07:08:09 +09:00") ; }
    };
}

test_datetime_sub_days!();