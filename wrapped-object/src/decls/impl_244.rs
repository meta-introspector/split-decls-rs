macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        CoffSymbol!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > read :: private :: Sealed for CoffSymbol < 'data , 'file , R , Coff > { }
    };
}

impl_244!()