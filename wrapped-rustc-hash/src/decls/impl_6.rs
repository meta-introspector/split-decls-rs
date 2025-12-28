macro_rules! deps {
    () => {
        FxRandomState!();
        FxHasher!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl core :: hash :: BuildHasher for FxRandomState { type Hasher = FxHasher ; fn build_hasher (& self) -> Self :: Hasher { FxHasher :: with_seed (self . seed) } }
    };
}

impl_6!();