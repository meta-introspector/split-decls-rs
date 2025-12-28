macro_rules! std {
    () => {
        # [cfg (feature = "std_rng")] mod std ;
    };
}

std!();