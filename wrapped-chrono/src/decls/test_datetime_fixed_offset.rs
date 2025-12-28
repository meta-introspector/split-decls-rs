macro_rules! deps {
    () => {
        NaiveDate!();
        FixedOffset!();
        Utc!();
    };
}

macro_rules! test_datetime_fixed_offset {
    () => {
        deps!();
        # [test] fn test_datetime_fixed_offset () { let naivedatetime = NaiveDate :: from_ymd_opt (2023 , 1 , 1) . unwrap () . and_hms_opt (0 , 0 , 0) . unwrap () ; let datetime = Utc . from_utc_datetime (& naivedatetime) ; let fixed_utc = FixedOffset :: east_opt (0) . unwrap () ; assert_eq ! (datetime . fixed_offset () , fixed_utc . from_local_datetime (& naivedatetime) . unwrap ()) ; let fixed_offset = FixedOffset :: east_opt (3600) . unwrap () ; let datetime_fixed = fixed_offset . from_local_datetime (& naivedatetime) . unwrap () ; assert_eq ! (datetime_fixed . fixed_offset () , datetime_fixed) ; }
    };
}

test_datetime_fixed_offset!()