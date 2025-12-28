macro_rules! deps {
    () => {
        FxHasher!();
        FxBuildHasher!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl BuildHasher for FxBuildHasher { type Hasher = FxHasher ; fn build_hasher (& self) -> FxHasher { FxHasher :: default () } }
    };
}

impl_19!()