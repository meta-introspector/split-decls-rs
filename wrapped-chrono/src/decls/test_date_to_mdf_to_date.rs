macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_to_mdf_to_date {
    () => {
        deps!();
        # [test] fn test_date_to_mdf_to_date () { for (year , year_flags , _) in YEAR_FLAGS { for ordinal in 1 ..= year_flags . ndays () { let date = NaiveDate :: from_yo_opt (year , ordinal) . unwrap () ; assert_eq ! (date , NaiveDate :: from_mdf (date . year () , date . mdf ()) . unwrap ()) ; } } }
    };
}

test_date_to_mdf_to_date!();