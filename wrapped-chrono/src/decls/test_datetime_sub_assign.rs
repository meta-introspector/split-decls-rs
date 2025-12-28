macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveDate!();
        FixedOffset!();
    };
}

macro_rules! test_datetime_sub_assign {
    () => {
        deps!();
        # [test] fn test_datetime_sub_assign () { let naivedatetime = NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap () . and_hms_opt (12 , 0 , 0) . unwrap () ; let datetime = naivedatetime . and_utc () ; let mut datetime_sub = datetime ; datetime_sub -= TimeDelta :: try_minutes (90) . unwrap () ; assert_eq ! (datetime_sub , datetime - TimeDelta :: try_minutes (90) . unwrap ()) ; let timezone = FixedOffset :: east_opt (60 * 60) . unwrap () ; let datetime = datetime . with_timezone (& timezone) ; let datetime_sub = datetime_sub . with_timezone (& timezone) ; assert_eq ! (datetime_sub , datetime - TimeDelta :: try_minutes (90) . unwrap ()) ; let timezone = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () ; let datetime = datetime . with_timezone (& timezone) ; let datetime_sub = datetime_sub . with_timezone (& timezone) ; assert_eq ! (datetime_sub , datetime - TimeDelta :: try_minutes (90) . unwrap ()) ; }
    };
}

test_datetime_sub_assign!()