macro_rules! TV_MIN_SECONDS {
    () => {
        const TV_MIN_SECONDS : i64 = - TV_MAX_SECONDS ;
    };
}

TV_MIN_SECONDS!()