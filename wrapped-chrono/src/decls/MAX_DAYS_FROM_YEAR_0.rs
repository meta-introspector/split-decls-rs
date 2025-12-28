macro_rules! MAX_DAYS_FROM_YEAR_0 {
    () => {
        const MAX_DAYS_FROM_YEAR_0 : i32 = (MAX_YEAR + 1) * 365 + MAX_YEAR / 4 - MAX_YEAR / 100 + MAX_YEAR / 400 ;
    };
}

MAX_DAYS_FROM_YEAR_0!();