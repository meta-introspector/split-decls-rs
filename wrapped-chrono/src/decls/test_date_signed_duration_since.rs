macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
    };
}

macro_rules! test_date_signed_duration_since {
    () => {
        deps!();
        # [test] fn test_date_signed_duration_since () { fn check (lhs : Option < NaiveDate > , rhs : Option < NaiveDate > , delta : TimeDelta) { assert_eq ! (lhs . unwrap () . signed_duration_since (rhs . unwrap ()) , delta) ; assert_eq ! (rhs . unwrap () . signed_duration_since (lhs . unwrap ()) , - delta) ; } let ymd = NaiveDate :: from_ymd_opt ; check (ymd (2014 , 1 , 1) , ymd (2014 , 1 , 1) , TimeDelta :: zero ()) ; check (ymd (2014 , 1 , 2) , ymd (2014 , 1 , 1) , TimeDelta :: try_days (1) . unwrap ()) ; check (ymd (2014 , 12 , 31) , ymd (2014 , 1 , 1) , TimeDelta :: try_days (364) . unwrap ()) ; check (ymd (2015 , 1 , 3) , ymd (2014 , 1 , 1) , TimeDelta :: try_days (365 + 2) . unwrap ()) ; check (ymd (2018 , 1 , 1) , ymd (2014 , 1 , 1) , TimeDelta :: try_days (365 * 4 + 1) . unwrap ()) ; check (ymd (2414 , 1 , 1) , ymd (2014 , 1 , 1) , TimeDelta :: try_days (365 * 400 + 97) . unwrap ()) ; check (ymd (MAX_YEAR , 12 , 31) , ymd (0 , 1 , 1) , TimeDelta :: try_days (MAX_DAYS_FROM_YEAR_0 as i64) . unwrap () ,) ; check (ymd (MIN_YEAR , 1 , 1) , ymd (0 , 1 , 1) , TimeDelta :: try_days (MIN_DAYS_FROM_YEAR_0 as i64) . unwrap () ,) ; }
    };
}

test_date_signed_duration_since!()