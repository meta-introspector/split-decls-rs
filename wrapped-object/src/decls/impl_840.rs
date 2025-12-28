macro_rules! deps {
    () => {
        Result!();
        FileHeader!();
        ReadRef!();
        XcoffRelocationIterator!();
    };
}

macro_rules! impl_840 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > fmt :: Debug for XcoffRelocationIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("XcoffRelocationIterator") . finish () } }
    };
}

impl_840!();