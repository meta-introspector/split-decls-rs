macro_rules! deps {
    () => {
        ParValues!();
        ParValuesMut!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < K : Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParValuesMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParValues { inner : self . inner . clone () , marker : PhantomData , } . fmt (f) } }
    };
}

impl_141!();