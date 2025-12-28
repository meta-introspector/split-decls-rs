macro_rules! rand {
    () => {
        # [cfg (feature = "rand_core")] mod rand ;
    };
}

rand!();