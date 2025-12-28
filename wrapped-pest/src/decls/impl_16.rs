macro_rules! deps {
    () => {
        RuleType!();
        Error!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Display for Error < R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . format ()) } }
    };
}

impl_16!()