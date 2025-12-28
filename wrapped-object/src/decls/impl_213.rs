macro_rules! deps {
    () => {
        CoffSegment!();
        CoffHeader!();
        ReadRef!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > read :: private :: Sealed for CoffSegment < 'data , 'file , R , Coff > { }
    };
}

impl_213!();