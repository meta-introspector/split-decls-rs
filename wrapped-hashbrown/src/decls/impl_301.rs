macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < K , V > Default for IterMut < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , marker : PhantomData , } } }
    };
}

impl_301!();