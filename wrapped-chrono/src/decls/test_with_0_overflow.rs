macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_with_0_overflow {
    () => {
        deps!();
        # [test] fn test_with_0_overflow () { let dt = NaiveDate :: from_ymd_opt (2023 , 4 , 18) . unwrap () ; assert ! (dt . with_month0 (4294967295) . is_none ()) ; assert ! (dt . with_day0 (4294967295) . is_none ()) ; assert ! (dt . with_ordinal0 (4294967295) . is_none ()) ; }
    };
}

test_with_0_overflow!();