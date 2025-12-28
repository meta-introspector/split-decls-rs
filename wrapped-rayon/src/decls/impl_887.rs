macro_rules! deps {
    () => {
        TakeAnyWhile!();
    };
}

macro_rules! impl_887 {
    () => {
        deps!();
        impl < I : fmt :: Debug , P > fmt :: Debug for TakeAnyWhile < I , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TakeAnyWhile") . field ("base" , & self . base) . finish () } }
    };
}

impl_887!();