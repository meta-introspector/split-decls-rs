macro_rules! DAYS_PER_4_YEARS {
    () => {
        # [doc = " Number of days in 4 years (including 1 leap year)"] const DAYS_PER_4_YEARS : i64 = DAYS_PER_NORMAL_YEAR * 4 + 1 ;
    };
}

DAYS_PER_4_YEARS!()