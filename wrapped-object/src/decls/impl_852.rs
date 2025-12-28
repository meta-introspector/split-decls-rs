macro_rules! deps {
    () => {
        ReadRef!();
        XcoffComdat!();
        FileHeader!();
    };
}

macro_rules! impl_852 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > read :: private :: Sealed for XcoffComdat < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_852!()