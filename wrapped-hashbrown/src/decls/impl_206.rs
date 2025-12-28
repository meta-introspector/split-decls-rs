macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < T > Clone for ParIter < '_ , T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_206!();