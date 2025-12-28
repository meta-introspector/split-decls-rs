macro_rules! deps {
    () => {
        IterHash!();
        IterHashMut!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IterHashMut < '_ , T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (IterHash { inner : self . inner . clone () , marker : PhantomData , }) . finish () } }
    };
}

impl_514!();