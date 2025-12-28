macro_rules! deps {
    () => {
        FoldHasher!();
        SharedSeed!();
        FixedState!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl BuildHasher for FixedState { type Hasher = FoldHasher < 'static > ; # [inline (always)] fn build_hasher (& self) -> FoldHasher < 'static > { FoldHasher :: with_seed (self . per_hasher_seed , SharedSeed :: global_fixed ()) } }
    };
}

impl_13!();