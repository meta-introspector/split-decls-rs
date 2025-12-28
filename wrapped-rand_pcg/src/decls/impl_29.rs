macro_rules! deps {
    () => {
        Lcg64Xsh32!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl SeedableRng for Lcg64Xsh32 { type Seed = [u8 ; 16] ; # [doc = " We use a single 127-bit seed to initialise the state and select a stream."] # [doc = " One `seed` bit (lowest bit of `seed[8]`) is ignored."] fn from_seed (seed : Self :: Seed) -> Self { let mut seed_u64 = [0u64 ; 2] ; le :: read_u64_into (& seed , & mut seed_u64) ; Lcg64Xsh32 :: from_state_incr (seed_u64 [0] , seed_u64 [1] | 1) } }
    };
}

impl_29!()