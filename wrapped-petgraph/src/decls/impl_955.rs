macro_rules! deps {
    () => {
        NoPretty!();
    };
}

macro_rules! impl_955 {
    () => {
        deps!();
        impl < T > fmt :: Debug for NoPretty < T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{:?}" , self . 0) } }
    };
}

impl_955!();