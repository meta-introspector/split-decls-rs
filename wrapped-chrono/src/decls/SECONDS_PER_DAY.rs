macro_rules! SECONDS_PER_DAY {
    () => {
        # [doc = " Number of seconds in one day"] const SECONDS_PER_DAY : i64 = SECONDS_PER_HOUR * HOURS_PER_DAY ;
    };
}

SECONDS_PER_DAY!()