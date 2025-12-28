macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_878 {
    () => {
        deps!();
        impl < Fut > Debug for FuturesUnordered < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "FuturesUnordered {{ ... }}") } }
    };
}

impl_878!();