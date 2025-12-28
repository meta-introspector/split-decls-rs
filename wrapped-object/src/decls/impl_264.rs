macro_rules! deps {
    () => {
        ReadRef!();
        CoffHeader!();
        CoffComdat!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > read :: private :: Sealed for CoffComdat < 'data , 'file , R , Coff > { }
    };
}

impl_264!();