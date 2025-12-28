macro_rules! deps {
    () => {
        SkipAnyWhile!();
    };
}

macro_rules! impl_836 {
    () => {
        deps!();
        impl < I : fmt :: Debug , P > fmt :: Debug for SkipAnyWhile < I , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SkipAnyWhile") . field ("base" , & self . base) . finish () } }
    };
}

impl_836!();