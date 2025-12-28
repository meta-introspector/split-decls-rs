macro_rules! TS_MIN_SECONDS {
    () => {
        const TS_MIN_SECONDS : i64 = - TS_MAX_SECONDS ;
    };
}

TS_MIN_SECONDS!();