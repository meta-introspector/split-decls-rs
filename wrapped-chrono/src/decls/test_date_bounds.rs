macro_rules! deps {
    () => {
        NaiveDate!();
        YearFlags!();
    };
}

macro_rules! test_date_bounds {
    () => {
        deps!();
        # [test] fn test_date_bounds () { let calculated_min = NaiveDate :: from_ymd_opt (MIN_YEAR , 1 , 1) . unwrap () ; let calculated_max = NaiveDate :: from_ymd_opt (MAX_YEAR , 12 , 31) . unwrap () ; assert ! (NaiveDate :: MIN == calculated_min , "`NaiveDate::MIN` should have year flag {:?}" , calculated_min . year_flags ()) ; assert ! (NaiveDate :: MAX == calculated_max , "`NaiveDate::MAX` should have year flag {:?} and ordinal {}" , calculated_max . year_flags () , calculated_max . ordinal ()) ; let maxsecs = NaiveDate :: MAX . signed_duration_since (NaiveDate :: MIN) . num_seconds () ; let maxsecs = maxsecs + 86401 ; assert ! (maxsecs < (1 << MAX_BITS) , "The entire `NaiveDate` range somehow exceeds 2^{MAX_BITS} seconds") ; const BEFORE_MIN : NaiveDate = NaiveDate :: BEFORE_MIN ; assert_eq ! (BEFORE_MIN . year_flags () , YearFlags :: from_year (BEFORE_MIN . year ())) ; assert_eq ! ((BEFORE_MIN . month () , BEFORE_MIN . day ()) , (12 , 31)) ; const AFTER_MAX : NaiveDate = NaiveDate :: AFTER_MAX ; assert_eq ! (AFTER_MAX . year_flags () , YearFlags :: from_year (AFTER_MAX . year ())) ; assert_eq ! ((AFTER_MAX . month () , AFTER_MAX . day ()) , (1 , 1)) ; }
    };
}

test_date_bounds!()