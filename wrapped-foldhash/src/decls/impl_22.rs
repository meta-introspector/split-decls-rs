macro_rules! deps {
    () => {
        FoldHasher!();
        SeedableRandomState!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl BuildHasher for SeedableRandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher { inner : self . inner . build_hasher () , } } }
    };
}

impl_22!();