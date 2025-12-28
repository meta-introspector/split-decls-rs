macro_rules! deps {
    () => {
        ParIter!();
        ParIterMut!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for ParIterMut < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ParIter { inner : self . inner . clone () , marker : PhantomData , } . fmt (f) } }
    };
}

impl_210!();