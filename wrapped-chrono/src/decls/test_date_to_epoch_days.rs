macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_to_epoch_days {
    () => {
        deps!();
        # [test] fn test_date_to_epoch_days () { assert_eq ! (NaiveDate :: from_ymd_opt (1970 , 1 , 1) . unwrap () . to_epoch_days () , 0) ; for year in - 9999 .. 10001 { assert_eq ! (NaiveDate :: from_ymd_opt (year , 1 , 1) . unwrap () . to_epoch_days () , NaiveDate :: from_ymd_opt (year - 1 , 12 , 31) . unwrap () . to_epoch_days () + 1) ; } }
    };
}

test_date_to_epoch_days!();