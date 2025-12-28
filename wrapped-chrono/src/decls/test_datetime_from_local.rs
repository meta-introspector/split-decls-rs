macro_rules! deps {
    () => {
        Utc!();
        NaiveDate!();
        DateTime!();
        FixedOffset!();
    };
}

macro_rules! test_datetime_from_local {
    () => {
        deps!();
        # [test] # [allow (deprecated)] fn test_datetime_from_local () { let naivedatetime_utc = NaiveDate :: from_ymd_opt (2000 , 1 , 12) . unwrap () . and_hms_opt (2 , 0 , 0) . unwrap () ; let datetime_utc = DateTime :: < Utc > :: from_utc (naivedatetime_utc , Utc) ; let timezone_east = FixedOffset :: east_opt (8 * 60 * 60) . unwrap () ; let naivedatetime_east = NaiveDate :: from_ymd_opt (2000 , 1 , 12) . unwrap () . and_hms_opt (10 , 0 , 0) . unwrap () ; let datetime_east = DateTime :: < FixedOffset > :: from_local (naivedatetime_east , timezone_east) ; let timezone_west = FixedOffset :: west_opt (7 * 60 * 60) . unwrap () ; let naivedatetime_west = NaiveDate :: from_ymd_opt (2000 , 1 , 11) . unwrap () . and_hms_opt (19 , 0 , 0) . unwrap () ; let datetime_west = DateTime :: < FixedOffset > :: from_local (naivedatetime_west , timezone_west) ; assert_eq ! (datetime_east , datetime_utc . with_timezone (& timezone_east)) ; assert_eq ! (datetime_west , datetime_utc . with_timezone (& timezone_west)) ; }
    };
}

test_datetime_from_local!()