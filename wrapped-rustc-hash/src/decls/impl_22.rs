macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl FxHasher { # [doc = " Creates a `fx` hasher with a given seed."] pub const fn with_seed (seed : usize) -> FxHasher { FxHasher { hash : seed } } # [doc = " Creates a default `fx` hasher."] pub const fn default () -> FxHasher { FxHasher { hash : 0 } } }
    };
}

impl_22!()