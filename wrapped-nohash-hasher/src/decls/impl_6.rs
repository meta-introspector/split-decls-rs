macro_rules! deps {
    () => {
        NoHashHasher!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T > Default for NoHashHasher < T > { # [cfg (debug_assertions)] fn default () -> Self { NoHashHasher (0 , false , PhantomData) } # [cfg (not (debug_assertions))] fn default () -> Self { NoHashHasher (0 , PhantomData) } }
    };
}

impl_6!();