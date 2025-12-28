macro_rules! deps {
    () => {
        ReadRef!();
        File!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > > read :: private :: Sealed for File < 'data , R > { }
    };
}

impl_125!()