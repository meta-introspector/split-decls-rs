macro_rules! xoshiro128plusplus {
    () => {
        # [cfg (feature = "small_rng")] mod xoshiro128plusplus ;
    };
}

xoshiro128plusplus!();