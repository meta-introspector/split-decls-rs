macro_rules! deps {
    () => {
        PeComdat!();
        ImageNtHeaders!();
        ReadRef!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > read :: private :: Sealed for PeComdat < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { }
    };
}

impl_641!();