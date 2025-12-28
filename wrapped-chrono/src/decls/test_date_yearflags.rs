macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_yearflags {
    () => {
        deps!();
        # [test] fn test_date_yearflags () { for (year , year_flags , _) in YEAR_FLAGS { assert_eq ! (NaiveDate :: from_yo_opt (year , 1) . unwrap () . year_flags () , year_flags) ; } }
    };
}

test_date_yearflags!()