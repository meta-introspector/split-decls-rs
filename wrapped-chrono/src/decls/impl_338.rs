macro_rules! deps {
    () => {
        NaiveDate!();
        YearFlags!();
        Error!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        # [cfg (all (feature = "arbitrary" , feature = "std"))] impl arbitrary :: Arbitrary < '_ > for NaiveDate { fn arbitrary (u : & mut arbitrary :: Unstructured) -> arbitrary :: Result < NaiveDate > { let year = u . int_in_range (MIN_YEAR ..= MAX_YEAR) ? ; let max_days = YearFlags :: from_year (year) . ndays () ; let ord = u . int_in_range (1 ..= max_days) ? ; NaiveDate :: from_yo_opt (year , ord) . ok_or (arbitrary :: Error :: IncorrectFormat) } }
    };
}

impl_338!()