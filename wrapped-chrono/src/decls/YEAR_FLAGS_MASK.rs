macro_rules! YEAR_FLAGS_MASK {
    () => {
        const YEAR_FLAGS_MASK : i32 = LEAP_YEAR_MASK | WEEKDAY_FLAGS_MASK ;
    };
}

YEAR_FLAGS_MASK!();