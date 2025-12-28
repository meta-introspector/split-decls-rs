macro_rules! deps {
    () => {
        ImageNtHeaders!();
        ReadRef!();
        PeSegment!();
    };
}

macro_rules! impl_664 {
    () => {
        deps!();
        impl < 'data , 'file , Pe , R > read :: private :: Sealed for PeSegment < 'data , 'file , Pe , R > where Pe : ImageNtHeaders , R : ReadRef < 'data > , { }
    };
}

impl_664!();