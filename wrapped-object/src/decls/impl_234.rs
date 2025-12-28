macro_rules! deps {
    () => {
        CoffSymbolTable!();
        ReadRef!();
        CoffHeader!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > read :: private :: Sealed for CoffSymbolTable < 'data , 'file , R , Coff > { }
    };
}

impl_234!();