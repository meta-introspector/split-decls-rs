macro_rules! deps {
    () => {
        SeedableRandomState!();
        FoldHasher!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl BuildHasher for SeedableRandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher :: with_seed (self . per_hasher_seed , self . shared_seed) } }
    };
}

impl_9!()