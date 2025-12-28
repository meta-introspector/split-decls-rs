macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < K , V , P > Key < K , V , P > { # [allow (clippy :: new_without_default , reason = "this a const fn, so it can't be default yet. See <https://github.com/rust-lang/rust/issues/63065>")] pub (crate) const fn new () -> Key < K , V , P > { Key { _phantom : PhantomData } } }
    };
}

impl_130!()