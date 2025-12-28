macro_rules! deps {
    () => {
        FxHasher!();
        FxSeededState!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl core :: hash :: BuildHasher for FxSeededState { type Hasher = FxHasher ; fn build_hasher (& self) -> Self :: Hasher { FxHasher :: with_seed (self . seed) } }
    };
}

impl_14!();