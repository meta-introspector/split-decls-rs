macro_rules! deps {
    () => {
        XcoffSymbol!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! impl_824 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > read :: private :: Sealed for XcoffSymbol < 'data , 'file , Xcoff , R > { }
    };
}

impl_824!();