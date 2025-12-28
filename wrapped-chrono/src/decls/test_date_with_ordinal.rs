macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_with_ordinal {
    () => {
        deps!();
        # [test] fn test_date_with_ordinal () { let d = NaiveDate :: from_ymd_opt (2000 , 5 , 5) . unwrap () ; assert_eq ! (d . with_ordinal (0) , None) ; assert_eq ! (d . with_ordinal (1) , Some (NaiveDate :: from_ymd_opt (2000 , 1 , 1) . unwrap ())) ; assert_eq ! (d . with_ordinal (60) , Some (NaiveDate :: from_ymd_opt (2000 , 2 , 29) . unwrap ())) ; assert_eq ! (d . with_ordinal (61) , Some (NaiveDate :: from_ymd_opt (2000 , 3 , 1) . unwrap ())) ; assert_eq ! (d . with_ordinal (366) , Some (NaiveDate :: from_ymd_opt (2000 , 12 , 31) . unwrap ())) ; assert_eq ! (d . with_ordinal (367) , None) ; assert_eq ! (d . with_ordinal ((1 << 28) | 60) , None) ; let d = NaiveDate :: from_ymd_opt (1999 , 5 , 5) . unwrap () ; assert_eq ! (d . with_ordinal (366) , None) ; assert_eq ! (d . with_ordinal (u32 :: MAX) , None) ; }
    };
}

test_date_with_ordinal!()