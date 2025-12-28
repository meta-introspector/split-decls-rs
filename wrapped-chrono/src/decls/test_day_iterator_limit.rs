macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_day_iterator_limit {
    () => {
        deps!();
        # [test] fn test_day_iterator_limit () { assert_eq ! (NaiveDate :: from_ymd_opt (MAX_YEAR , 12 , 29) . unwrap () . iter_days () . take (4) . count () , 2) ; assert_eq ! (NaiveDate :: from_ymd_opt (MIN_YEAR , 1 , 3) . unwrap () . iter_days () . rev () . take (4) . count () , 2) ; }
    };
}

test_day_iterator_limit!();