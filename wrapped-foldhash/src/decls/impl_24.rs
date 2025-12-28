macro_rules! deps {
    () => {
        FixedState!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl FixedState { # [doc = " Creates a [`FixedState`] with the given per-hasher seed."] # [inline (always)] pub const fn with_seed (per_hasher_seed : u64) -> Self { Self { inner : fast :: FixedState :: with_seed (folded_multiply (per_hasher_seed , ARBITRARY4)) , } } }
    };
}

impl_24!();