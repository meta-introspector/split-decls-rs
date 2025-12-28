macro_rules! deps {
    () => {
        RandomState!();
        FoldHasher!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl BuildHasher for RandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher { inner : self . inner . build_hasher () , } } }
    };
}

impl_19!()