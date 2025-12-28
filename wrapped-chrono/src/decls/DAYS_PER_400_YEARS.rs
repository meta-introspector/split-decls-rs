macro_rules! DAYS_PER_400_YEARS {
    () => {
        # [doc = " Number of days in 400 years (including 97 leap years)"] const DAYS_PER_400_YEARS : i64 = DAYS_PER_NORMAL_YEAR * 400 + 97 ;
    };
}

DAYS_PER_400_YEARS!()