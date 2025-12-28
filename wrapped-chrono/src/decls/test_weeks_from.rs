macro_rules! deps {
    () => {
        Weekday!();
        NaiveDate!();
        Days!();
    };
}

macro_rules! test_weeks_from {
    () => {
        deps!();
        # [test] fn test_weeks_from () { assert_eq ! (NaiveDate :: parse_from_str ("2020-01-0" , "%Y-%W-%w") . ok () , NaiveDate :: from_ymd_opt (2020 , 1 , 12) ,) ; assert_eq ! (NaiveDate :: parse_from_str ("2019-01-0" , "%Y-%W-%w") . ok () , NaiveDate :: from_ymd_opt (2019 , 1 , 13) ,) ; for (y , starts_on) in & [(2019 , Weekday :: Tue) , (2020 , Weekday :: Wed) , (2021 , Weekday :: Fri) , (2022 , Weekday :: Sat) , (2023 , Weekday :: Sun) , (2024 , Weekday :: Mon) , (2025 , Weekday :: Wed) , (2026 , Weekday :: Thu) ,] { for day in & [Weekday :: Mon , Weekday :: Tue , Weekday :: Wed , Weekday :: Thu , Weekday :: Fri , Weekday :: Sat , Weekday :: Sun ,] { assert_eq ! (NaiveDate :: from_ymd_opt (* y , 1 , 1) . map (| d | d . weeks_from (* day)) , Some (if day == starts_on { 1 } else { 0 })) ; assert ! ([52 , 53] . contains (& NaiveDate :: from_ymd_opt (* y , 12 , 31) . unwrap () . weeks_from (* day)) ,) ; } } let base = NaiveDate :: from_ymd_opt (2019 , 1 , 1) . unwrap () ; for day in & [Weekday :: Mon , Weekday :: Tue , Weekday :: Wed , Weekday :: Thu , Weekday :: Fri , Weekday :: Sat , Weekday :: Sun ,] { for dplus in 1 .. (400 * 366) { assert ! ((base + Days :: new (dplus)) . weeks_from (* day) < 54) } } }
    };
}

test_weeks_from!();