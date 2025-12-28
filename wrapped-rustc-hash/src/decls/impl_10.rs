macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for FxHasher { # [inline] fn default () -> FxHasher { Self :: default () } }
    };
}

impl_10!()