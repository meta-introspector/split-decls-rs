macro_rules! deps {
    () => {
        FxBuildHasher!();
        FxHasher!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl BuildHasher for FxBuildHasher { type Hasher = FxHasher ; fn build_hasher (& self) -> FxHasher { FxHasher :: default () } }
    };
}

impl_32!()