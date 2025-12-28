macro_rules! deps {
    () => {
        StdRng!();
        Rng!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl SeedableRng for StdRng { type Seed = [u8 ; 32] ; # [inline (always)] fn from_seed (seed : Self :: Seed) -> Self { StdRng (Rng :: from_seed (seed)) } }
    };
}

impl_246!()