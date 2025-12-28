macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        CoffFile!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > , Coff : CoffHeader > read :: private :: Sealed for CoffFile < 'data , R , Coff > { }
    };
}

impl_198!()