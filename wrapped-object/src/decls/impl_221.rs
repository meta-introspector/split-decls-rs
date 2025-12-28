macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        CoffSection!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > read :: private :: Sealed for CoffSection < 'data , 'file , R , Coff > { }
    };
}

impl_221!();