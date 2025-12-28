macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! feed_rand_16mib {
    () => {
        deps!();
        # [doc = " Feed ~1 MiB of pseudorandom data to an updatable state."] pub fn feed_rand_16mib < D : crate :: Update > (d : & mut D) { let buf = & mut [0u8 ; 1024] ; let mut rng = rng :: RNG ; let n = 16 * (1 << 20) / buf . len () ; for _ in 0 .. n { rng . fill (buf) ; d . update (buf) ; d . update (& [42]) ; } }
    };
}

feed_rand_16mib!()