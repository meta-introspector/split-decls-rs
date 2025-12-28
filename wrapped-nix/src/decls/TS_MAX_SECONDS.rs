macro_rules! TS_MAX_SECONDS {
    () => {
        # [cfg (target_pointer_width = "32")] const TS_MAX_SECONDS : i64 = isize :: MAX as i64 ;
    };
}

TS_MAX_SECONDS!();