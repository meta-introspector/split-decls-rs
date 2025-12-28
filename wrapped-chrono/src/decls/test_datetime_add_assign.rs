macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
        FixedOffset!();
    };
}

macro_rules! test_datetime_add_assign {
    () => {
        deps!();
        # [test] fn test_datetime_add_assign () { let naivedatetime = NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap () . and_hms_opt (0 , 0 , 0) . unwrap () ; let datetime = naivedatetime . and_utc () ; let mut datetime_add = datetime ; datetime_add += TimeDelta :: try_seconds (60) . unwrap () ; assert_eq ! (datetime_add , datetime + TimeDelta :: try_seconds (60) . unwrap ()) ; let timezone = FixedOffset :: east_opt (60 * 60) . unwrap () ; let datetime = datetime . with_timezone (& timezone) ; let datetime_add = datetime_add . with_timezone (& timezone) ; assert_eq ! (datetime_add , datetime + TimeDelta :: try_seconds (60) . unwrap ()) ; let timezone = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () ; let datetime = datetime . with_timezone (& timezone) ; let datetime_add = datetime_add . with_timezone (& timezone) ; assert_eq ! (datetime_add , datetime + TimeDelta :: try_seconds (60) . unwrap ()) ; }
    };
}

test_datetime_add_assign!();