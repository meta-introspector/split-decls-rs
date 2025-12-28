macro_rules! deps {
    () => {
        ParIter!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < K , V > Clone for ParIter < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Self { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_126!()