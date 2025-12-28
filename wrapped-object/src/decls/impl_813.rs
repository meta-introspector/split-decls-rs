macro_rules! deps {
    () => {
        XcoffSymbolTable!();
        ReadRef!();
        FileHeader!();
    };
}

macro_rules! impl_813 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > read :: private :: Sealed for XcoffSymbolTable < 'data , 'file , Xcoff , R > { }
    };
}

impl_813!();