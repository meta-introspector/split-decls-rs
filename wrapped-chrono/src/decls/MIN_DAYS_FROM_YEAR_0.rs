macro_rules! MIN_DAYS_FROM_YEAR_0 {
    () => {
        const MIN_DAYS_FROM_YEAR_0 : i32 = MIN_YEAR * 365 + MIN_YEAR / 4 - MIN_YEAR / 100 + MIN_YEAR / 400 ;
    };
}

MIN_DAYS_FROM_YEAR_0!();