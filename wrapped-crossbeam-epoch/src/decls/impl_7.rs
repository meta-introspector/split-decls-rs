macro_rules! deps {
    () => {
        Pointer!();
        CompareExchangeError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T , P : Pointer < T > + fmt :: Debug > fmt :: Debug for CompareExchangeError < '_ , T , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CompareExchangeError") . field ("current" , & self . current) . field ("new" , & self . new) . finish () } }
    };
}

impl_7!();