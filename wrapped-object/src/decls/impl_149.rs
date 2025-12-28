macro_rules! deps {
    () => {
        Comdat!();
        ReadRef!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > read :: private :: Sealed for Comdat < 'data , 'file , R > { }
    };
}

impl_149!()