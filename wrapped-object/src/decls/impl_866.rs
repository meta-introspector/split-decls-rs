macro_rules! deps {
    () => {
        XcoffSegment!();
        FileHeader!();
        ReadRef!();
    };
}

macro_rules! impl_866 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > XcoffSegment < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { }
    };
}

impl_866!();