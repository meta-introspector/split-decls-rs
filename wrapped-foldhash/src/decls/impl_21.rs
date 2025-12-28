macro_rules! deps {
    () => {
        SeedableRandomState!();
        RandomState!();
        SharedSeed!();
        FixedState!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl SeedableRandomState { # [doc = " Generates a random [`SeedableRandomState`], similar to [`RandomState`]."] # [inline (always)] pub fn random () -> Self { Self { inner : fast :: SeedableRandomState :: random () , } } # [doc = " Generates a fixed [`SeedableRandomState`], similar to [`FixedState`]."] # [inline (always)] pub fn fixed () -> Self { Self { inner : fast :: SeedableRandomState :: fixed () , } } # [doc = " Generates a [`SeedableRandomState`] with the given per-hasher seed"] # [doc = " and [`SharedSeed`]."] # [inline (always)] pub fn with_seed (per_hasher_seed : u64 , shared_seed : & 'static SharedSeed) -> Self { Self { inner : fast :: SeedableRandomState :: with_seed (folded_multiply (per_hasher_seed , ARBITRARY4) , shared_seed ,) , } } }
    };
}

impl_21!()