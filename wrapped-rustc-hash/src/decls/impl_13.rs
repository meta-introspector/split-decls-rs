macro_rules! deps {
    () => {
        FxSeededState!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl FxSeededState { # [doc = " Constructs a new `FxSeededState` that is initialized with a `seed`."] pub const fn with_seed (seed : usize) -> FxSeededState { Self { seed } } }
    };
}

impl_13!()