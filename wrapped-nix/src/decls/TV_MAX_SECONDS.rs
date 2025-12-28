macro_rules! TV_MAX_SECONDS {
    () => {
        # [cfg (target_pointer_width = "32")] const TV_MAX_SECONDS : i64 = isize :: MAX as i64 ;
    };
}

TV_MAX_SECONDS!();