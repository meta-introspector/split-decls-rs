macro_rules! deps {
    () => {
        ReadRef!();
        FileHeader!();
        XcoffFile!();
    };
}

macro_rules! impl_779 {
    () => {
        deps!();
        impl < 'data , Xcoff , R > read :: private :: Sealed for XcoffFile < 'data , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_779!()