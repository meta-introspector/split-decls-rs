macro_rules! deps {
    () => {
        FileHeader!();
        XcoffSegment!();
        ReadRef!();
    };
}

macro_rules! impl_867 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > read :: private :: Sealed for XcoffSegment < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_867!();