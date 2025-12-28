macro_rules! deps {
    () => {
        NaiveDate!();
        FixedOffset!();
        NaiveTime!();
        Months!();
        NaiveDateTime!();
    };
}

macro_rules! test_min_max_add_months {
    () => {
        deps!();
        # [test] fn test_min_max_add_months () { let offset_min = FixedOffset :: west_opt (2 * 60 * 60) . unwrap () ; let beyond_min = offset_min . from_utc_datetime (& NaiveDateTime :: MIN) ; let offset_max = FixedOffset :: east_opt (2 * 60 * 60) . unwrap () ; let beyond_max = offset_max . from_utc_datetime (& NaiveDateTime :: MAX) ; let max_time = NaiveTime :: from_hms_nano_opt (23 , 59 , 59 , 999_999_999) . unwrap () ; assert_eq ! (beyond_min . checked_add_months (Months :: new (0)) , Some (beyond_min)) ; assert_eq ! (beyond_min . checked_add_months (Months :: new (1)) , Some (offset_min . from_utc_datetime (& (NaiveDate :: MIN + Months (1)) . and_time (NaiveTime :: MIN)))) ; assert_eq ! (beyond_min . checked_sub_months (Months :: new (0)) , Some (beyond_min)) ; assert_eq ! (beyond_min . checked_sub_months (Months :: new (1)) , None) ; assert_eq ! (beyond_max . checked_add_months (Months :: new (0)) , Some (beyond_max)) ; assert_eq ! (beyond_max . checked_add_months (Months :: new (1)) , None) ; assert_eq ! (beyond_max . checked_sub_months (Months :: new (0)) , Some (beyond_max)) ; assert_eq ! (beyond_max . checked_sub_months (Months :: new (1)) , Some (offset_max . from_utc_datetime (& (NaiveDate :: MAX - Months (1)) . and_time (max_time)))) ; }
    };
}

test_min_max_add_months!();