macro_rules! SECONDS_PER_WEEK {
    () => {
        # [doc = " Number of seconds in one week"] pub (crate) const SECONDS_PER_WEEK : i64 = SECONDS_PER_DAY * DAYS_PER_WEEK ;
    };
}

SECONDS_PER_WEEK!()