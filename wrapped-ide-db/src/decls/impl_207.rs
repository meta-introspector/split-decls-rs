macro_rules! deps {
    () => {
        Result!();
        SymbolIndex!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl fmt :: Debug for SymbolIndex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("SymbolIndex") . field ("n_symbols" , & self . symbols . len ()) . finish () } }
    };
}

impl_207!()