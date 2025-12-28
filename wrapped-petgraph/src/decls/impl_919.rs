macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_919 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for Ptr < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_919!()