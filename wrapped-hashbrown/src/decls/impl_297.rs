macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < K , V > Default for Iter < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , marker : PhantomData , } } }
    };
}

impl_297!();