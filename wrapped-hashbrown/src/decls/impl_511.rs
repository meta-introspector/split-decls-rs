macro_rules! deps {
    () => {
        IterHashMut!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl < T > Default for IterHashMut < '_ , T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { IterHashMut { inner : Default :: default () , marker : PhantomData , } } }
    };
}

impl_511!()