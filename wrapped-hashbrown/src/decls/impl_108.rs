macro_rules! deps {
    () => {
        RawIterHash!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T > Clone for RawIterHash < T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , _marker : PhantomData , } } }
    };
}

impl_108!()