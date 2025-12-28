macro_rules! deps {
    () => {
        XcoffSection!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! impl_796 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > read :: private :: Sealed for XcoffSection < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_796!();