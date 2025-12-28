macro_rules! deps {
    () => {
        FixedState!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl FixedState { # [doc = " Creates a [`FixedState`] with the given per-hasher-seed."] # [inline (always)] pub const fn with_seed (per_hasher_seed : u64) -> Self { Self { per_hasher_seed : per_hasher_seed ^ ARBITRARY3 , } } }
    };
}

impl_11!();