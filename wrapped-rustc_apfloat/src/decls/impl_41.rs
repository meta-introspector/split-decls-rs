macro_rules! deps {
    () => {
        IeeeFloat!();
        Semantics!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < S : Semantics > fmt :: Debug for IeeeFloat < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}({:?} | {}{:?} * 2^{})" , self , self . category () , if self . is_negative () { "-" } else { "+" } , self . sig , self . exp) } }
    };
}

impl_41!()