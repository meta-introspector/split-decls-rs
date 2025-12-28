macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl < T > Default for IterMut < '_ , T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { IterMut { inner : Default :: default () , marker : PhantomData , } } }
    };
}

impl_499!()