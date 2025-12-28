macro_rules! deps {
    () => {
        CoffSymbolIterator!();
        CoffHeader!();
        ReadRef!();
        Result!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > fmt :: Debug for CoffSymbolIterator < 'data , 'file , R , Coff > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("CoffSymbolIterator") . finish () } }
    };
}

impl_239!();