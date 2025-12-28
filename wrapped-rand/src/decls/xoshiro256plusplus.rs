macro_rules! xoshiro256plusplus {
    () => {
        # [cfg (feature = "small_rng")] mod xoshiro256plusplus ;
    };
}

xoshiro256plusplus!()