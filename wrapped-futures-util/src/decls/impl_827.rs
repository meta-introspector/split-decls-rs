macro_rules! deps {
    () => {
        FuturesOrdered!();
    };
}

macro_rules! impl_827 {
    () => {
        deps!();
        impl < Fut : Future > Debug for FuturesOrdered < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "FuturesOrdered {{ ... }}") } }
    };
}

impl_827!()