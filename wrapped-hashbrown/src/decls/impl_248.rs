macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < K , V > Clone for Iter < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Iter { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_248!()