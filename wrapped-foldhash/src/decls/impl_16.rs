macro_rules! deps {
    () => {
        FoldHasher!();
        SharedSeed!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > FoldHasher < 'a > { # [doc = " Initializes this [`FoldHasher`] with the given per-hasher seed and"] # [doc = " [`SharedSeed`]."] # [inline (always)] pub const fn with_seed (per_hasher_seed : u64 , shared_seed : & 'a SharedSeed) -> FoldHasher < 'a > { FoldHasher { inner : fast :: FoldHasher :: with_seed (per_hasher_seed , shared_seed) , } } }
    };
}

impl_16!();