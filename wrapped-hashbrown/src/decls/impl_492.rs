macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl < T > Default for Iter < '_ , T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Iter { inner : Default :: default () , marker : PhantomData , } } }
    };
}

impl_492!()