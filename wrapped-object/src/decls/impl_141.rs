macro_rules! deps {
    () => {
        ReadRef!();
        Section!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > read :: private :: Sealed for Section < 'data , 'file , R > { }
    };
}

impl_141!()