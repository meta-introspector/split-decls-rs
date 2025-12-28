macro_rules! deps {
    () => {
        Months!();
        FixedOffset!();
    };
}

macro_rules! test_datetime_add_months {
    () => {
        deps!();
        # [test] fn test_datetime_add_months () { let est = FixedOffset :: west_opt (5 * 60 * 60) . unwrap () ; let kst = FixedOffset :: east_opt (9 * 60 * 60) . unwrap () ; assert_eq ! (format ! ("{}" , est . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Months :: new (1)) , "2014-06-06 07:08:09 -05:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Months :: new (1)) , "2014-06-06 07:08:09 +09:00") ; assert_eq ! (format ! ("{}" , est . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Months :: new (5)) , "2014-10-06 07:08:09 -05:00") ; assert_eq ! (format ! ("{}" , kst . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () + Months :: new (5)) , "2014-10-06 07:08:09 +09:00") ; }
    };
}

test_datetime_add_months!()