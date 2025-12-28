macro_rules! deps {
    () => {
        FixedState!();
        FoldHasher!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl BuildHasher for FixedState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher { inner : self . inner . build_hasher () , } } }
    };
}

impl_25!();