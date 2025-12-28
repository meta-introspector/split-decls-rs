macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_weekday_with_yearflags {
    () => {
        deps!();
        # [test] fn test_weekday_with_yearflags () { for (year , year_flags , first_weekday) in YEAR_FLAGS { let first_day_of_year = NaiveDate :: from_yo_opt (year , 1) . unwrap () ; dbg ! (year) ; assert_eq ! (first_day_of_year . year_flags () , year_flags) ; assert_eq ! (first_day_of_year . weekday () , first_weekday) ; let mut prev = first_day_of_year . weekday () ; for ordinal in 2u32 ..= year_flags . ndays () { let date = NaiveDate :: from_yo_opt (year , ordinal) . unwrap () ; let expected = prev . succ () ; assert_eq ! (date . weekday () , expected) ; prev = expected ; } } }
    };
}

test_weekday_with_yearflags!()