macro_rules! deps {
    () => {
        ReadRef!();
        ImageNtHeaders!();
        PeSection!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > read :: private :: Sealed for PeSection < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { }
    };
}

impl_674!();