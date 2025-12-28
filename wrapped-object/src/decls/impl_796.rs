macro_rules! deps {
    () => {
        ReadRef!();
        XcoffSection!();
        FileHeader!();
    };
}

macro_rules! impl_796 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > read :: private :: Sealed for XcoffSection < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_796!()