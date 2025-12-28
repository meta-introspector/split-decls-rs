macro_rules! deps {
    () => {
        LocalFutureObj!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > fmt :: Debug for LocalFutureObj < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("LocalFutureObj") . finish () } }
    };
}

impl_35!();