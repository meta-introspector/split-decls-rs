macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveDate!();
    };
}

macro_rules! test_date_addassignment {
    () => {
        deps!();
        # [test] fn test_date_addassignment () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; let mut date = ymd (2016 , 10 , 1) ; date += TimeDelta :: try_days (10) . unwrap () ; assert_eq ! (date , ymd (2016 , 10 , 11)) ; date += TimeDelta :: try_days (30) . unwrap () ; assert_eq ! (date , ymd (2016 , 11 , 10)) ; }
    };
}

test_date_addassignment!()