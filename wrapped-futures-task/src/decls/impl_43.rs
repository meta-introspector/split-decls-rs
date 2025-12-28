macro_rules! deps {
    () => {
        FutureObj!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T > fmt :: Debug for FutureObj < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FutureObj") . finish () } }
    };
}

impl_43!()