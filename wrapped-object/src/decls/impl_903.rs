macro_rules! deps {
    () => {
        Result!();
        SymbolIndex!();
    };
}

macro_rules! impl_903 {
    () => {
        deps!();
        impl fmt :: Display for SymbolIndex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_903!();