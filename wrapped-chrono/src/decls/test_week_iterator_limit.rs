macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_week_iterator_limit {
    () => {
        deps!();
        # [test] fn test_week_iterator_limit () { assert_eq ! (NaiveDate :: from_ymd_opt (MAX_YEAR , 12 , 12) . unwrap () . iter_weeks () . take (4) . count () , 2) ; assert_eq ! (NaiveDate :: from_ymd_opt (MIN_YEAR , 1 , 15) . unwrap () . iter_weeks () . rev () . take (4) . count () , 2) ; }
    };
}

test_week_iterator_limit!()