macro_rules! SECONDS_PER_28_DAYS {
    () => {
        # [doc = " Number of seconds in 28 days"] const SECONDS_PER_28_DAYS : i64 = SECONDS_PER_DAY * 28 ;
    };
}

SECONDS_PER_28_DAYS!();