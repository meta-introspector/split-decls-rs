macro_rules! deps {
    () => {
        NaiveDate!();
        Weekday!();
    };
}

macro_rules! test_date_from_isoywd_and_iso_week {
    () => {
        deps!();
        # [test] fn test_date_from_isoywd_and_iso_week () { for year in 2000 .. 2401 { for week in 1 .. 54 { for & weekday in [Weekday :: Mon , Weekday :: Tue , Weekday :: Wed , Weekday :: Thu , Weekday :: Fri , Weekday :: Sat , Weekday :: Sun ,] . iter () { let d = NaiveDate :: from_isoywd_opt (year , week , weekday) ; if let Some (d) = d { assert_eq ! (d . weekday () , weekday) ; let w = d . iso_week () ; assert_eq ! (w . year () , year) ; assert_eq ! (w . week () , week) ; } } } } for year in 2000 .. 2401 { for month in 1 .. 13 { for day in 1 .. 32 { let d = NaiveDate :: from_ymd_opt (year , month , day) ; if let Some (d) = d { let w = d . iso_week () ; let d_ = NaiveDate :: from_isoywd_opt (w . year () , w . week () , d . weekday ()) ; assert_eq ! (d , d_ . unwrap ()) ; } } } } }
    };
}

test_date_from_isoywd_and_iso_week!();