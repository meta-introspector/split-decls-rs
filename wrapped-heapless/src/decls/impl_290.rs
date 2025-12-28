macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > fmt :: Debug for VecInner < T , LenT , S > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < [T] as fmt :: Debug > :: fmt (self , f) } }
    };
}

impl_290!();