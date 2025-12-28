macro_rules! global_rng {
    () => {
        # [cfg (feature = "std")] mod global_rng ;
    };
}

global_rng!()