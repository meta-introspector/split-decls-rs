macro_rules! deps {
    () => {
        ParIter!();
        ParIterMut!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug > fmt :: Debug for ParIterMut < '_ , K , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : self . inner . clone () , marker : PhantomData , } . fmt (f) } }
    };
}

impl_138!()