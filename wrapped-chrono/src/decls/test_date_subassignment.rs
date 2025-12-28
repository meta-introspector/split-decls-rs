macro_rules! deps {
    () => {
        TimeDelta!();
        NaiveDate!();
    };
}

macro_rules! test_date_subassignment {
    () => {
        deps!();
        # [test] fn test_date_subassignment () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; let mut date = ymd (2016 , 10 , 11) ; date -= TimeDelta :: try_days (10) . unwrap () ; assert_eq ! (date , ymd (2016 , 10 , 1)) ; date -= TimeDelta :: try_days (2) . unwrap () ; assert_eq ! (date , ymd (2016 , 9 , 29)) ; }
    };
}

test_date_subassignment!();