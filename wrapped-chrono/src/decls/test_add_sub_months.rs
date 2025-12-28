macro_rules! deps {
    () => {
        Months!();
        Utc!();
    };
}

macro_rules! test_add_sub_months {
    () => {
        deps!();
        # [test] fn test_add_sub_months () { let utc_dt = Utc . with_ymd_and_hms (2018 , 9 , 5 , 23 , 58 , 0) . unwrap () ; assert_eq ! (utc_dt + Months :: new (15) , Utc . with_ymd_and_hms (2019 , 12 , 5 , 23 , 58 , 0) . unwrap ()) ; let utc_dt = Utc . with_ymd_and_hms (2020 , 1 , 31 , 23 , 58 , 0) . unwrap () ; assert_eq ! (utc_dt + Months :: new (1) , Utc . with_ymd_and_hms (2020 , 2 , 29 , 23 , 58 , 0) . unwrap ()) ; assert_eq ! (utc_dt + Months :: new (2) , Utc . with_ymd_and_hms (2020 , 3 , 31 , 23 , 58 , 0) . unwrap ()) ; let utc_dt = Utc . with_ymd_and_hms (2018 , 9 , 5 , 23 , 58 , 0) . unwrap () ; assert_eq ! (utc_dt - Months :: new (15) , Utc . with_ymd_and_hms (2017 , 6 , 5 , 23 , 58 , 0) . unwrap ()) ; let utc_dt = Utc . with_ymd_and_hms (2020 , 3 , 31 , 23 , 58 , 0) . unwrap () ; assert_eq ! (utc_dt - Months :: new (1) , Utc . with_ymd_and_hms (2020 , 2 , 29 , 23 , 58 , 0) . unwrap ()) ; assert_eq ! (utc_dt - Months :: new (2) , Utc . with_ymd_and_hms (2020 , 1 , 31 , 23 , 58 , 0) . unwrap ()) ; }
    };
}

test_add_sub_months!();