macro_rules! DAYS_PER_100_YEARS {
    () => {
        # [doc = " Number of days in 100 years (including 24 leap years)"] const DAYS_PER_100_YEARS : i64 = DAYS_PER_NORMAL_YEAR * 100 + 24 ;
    };
}

DAYS_PER_100_YEARS!();