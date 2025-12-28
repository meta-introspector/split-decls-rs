macro_rules! deps {
    () => {
        ExtendedHasher!();
        StableHasher!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < H : ExtendedHasher + Default > Default for StableHasher < H > { # [doc = " Creates a new [`StableHasher`]."] # [doc = ""] # [doc = " To be used with the [`Hasher`] implementation and [`StableHasher::finish`]."] # [inline] fn default () -> Self { StableHasher { state : Default :: default () , } } }
    };
}

impl_31!();