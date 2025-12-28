macro_rules! deps {
    () => {
        ExtendedHasher!();
        StableHasher!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < H : ExtendedHasher + Default > StableHasher < H > { # [doc = " Creates a new [`StableHasher`]."] # [doc = ""] # [doc = " To be used with the [`Hasher`] implementation and [`StableHasher::finish`]."] # [inline] pub fn new () -> Self { Default :: default () } }
    };
}

impl_30!()