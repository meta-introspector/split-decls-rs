macro_rules! deps {
    () => {
        RandomState!();
        FoldHasher!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl BuildHasher for RandomState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher :: with_seed (self . per_hasher_seed , self . global_seed . get ()) } }
    };
}

impl_5!()