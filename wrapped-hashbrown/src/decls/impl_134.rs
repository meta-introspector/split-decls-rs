macro_rules! deps {
    () => {
        ParValues!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < K , V > Clone for ParValues < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_134!();