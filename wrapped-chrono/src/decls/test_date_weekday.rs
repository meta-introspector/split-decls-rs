macro_rules! deps {
    () => {
        Weekday!();
        NaiveDate!();
    };
}

macro_rules! test_date_weekday {
    () => {
        deps!();
        # [test] fn test_date_weekday () { assert_eq ! (NaiveDate :: from_ymd_opt (1582 , 10 , 15) . unwrap () . weekday () , Weekday :: Fri) ; assert_eq ! (NaiveDate :: from_ymd_opt (1875 , 5 , 20) . unwrap () . weekday () , Weekday :: Thu) ; assert_eq ! (NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap () . weekday () , Weekday :: Sat) ; }
    };
}

test_date_weekday!();