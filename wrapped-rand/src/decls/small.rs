macro_rules! small {
    () => {
        # [cfg (feature = "small_rng")] mod small ;
    };
}

small!();