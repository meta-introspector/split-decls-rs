macro_rules! deps {
    () => {
        Iter!();
        IterMut!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IterMut < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (Iter { inner : self . inner . clone () , marker : PhantomData , }) . finish () } }
    };
}

impl_503!();