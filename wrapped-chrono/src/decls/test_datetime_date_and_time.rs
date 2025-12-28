macro_rules! deps {
    () => {
        NaiveDate!();
        NaiveTime!();
        FixedOffset!();
        Utc!();
    };
}

macro_rules! test_datetime_date_and_time {
    () => {
        deps!();
        # [test] fn test_datetime_date_and_time () { let tz = FixedOffset :: east_opt (5 * 60 * 60) . unwrap () ; let d = tz . with_ymd_and_hms (2014 , 5 , 6 , 7 , 8 , 9) . unwrap () ; assert_eq ! (d . time () , NaiveTime :: from_hms_opt (7 , 8 , 9) . unwrap ()) ; assert_eq ! (d . date_naive () , NaiveDate :: from_ymd_opt (2014 , 5 , 6) . unwrap ()) ; let tz = FixedOffset :: east_opt (4 * 60 * 60) . unwrap () ; let d = tz . with_ymd_and_hms (2016 , 5 , 4 , 3 , 2 , 1) . unwrap () ; assert_eq ! (d . time () , NaiveTime :: from_hms_opt (3 , 2 , 1) . unwrap ()) ; assert_eq ! (d . date_naive () , NaiveDate :: from_ymd_opt (2016 , 5 , 4) . unwrap ()) ; let tz = FixedOffset :: west_opt (13 * 60 * 60) . unwrap () ; let d = tz . with_ymd_and_hms (2017 , 8 , 9 , 12 , 34 , 56) . unwrap () ; assert_eq ! (d . time () , NaiveTime :: from_hms_opt (12 , 34 , 56) . unwrap ()) ; assert_eq ! (d . date_naive () , NaiveDate :: from_ymd_opt (2017 , 8 , 9) . unwrap ()) ; let utc_d = Utc . with_ymd_and_hms (2017 , 8 , 9 , 12 , 34 , 56) . unwrap () ; assert ! (utc_d < d) ; }
    };
}

test_datetime_date_and_time!();