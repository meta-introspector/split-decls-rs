macro_rules! Never {
    () => {
        # [doc = " A cache implementation that doesn't do any caching."] pub struct Never ;
    };
}

Never!()