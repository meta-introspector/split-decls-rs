macro_rules! deps {
    () => {
        ParKeys!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < K , V > Clone for ParKeys < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_130!()