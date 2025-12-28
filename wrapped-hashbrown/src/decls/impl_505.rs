macro_rules! deps {
    () => {
        IterHash!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl < T > Default for IterHash < '_ , T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { IterHash { inner : Default :: default () , marker : PhantomData , } } }
    };
}

impl_505!()