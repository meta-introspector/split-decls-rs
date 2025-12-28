macro_rules! deps {
    () => {
        SymbolId!();
        Result!();
    };
}

macro_rules! impl_1129 {
    () => {
        deps!();
        impl < const DYNAMIC : bool > fmt :: Debug for SymbolId < DYNAMIC > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_1129!();