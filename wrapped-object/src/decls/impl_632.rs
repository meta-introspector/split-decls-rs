macro_rules! deps {
    () => {
        PeFile!();
        ImageNtHeaders!();
        ReadRef!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl < 'data , Pe , R > read :: private :: Sealed for PeFile < 'data , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { }
    };
}

impl_632!();