macro_rules! deps {
    () => {
        ReadRef!();
        FileHeader!();
        Result!();
        XcoffSymbolIterator!();
    };
}

macro_rules! impl_818 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff : FileHeader , R : ReadRef < 'data > > fmt :: Debug for XcoffSymbolIterator < 'data , 'file , Xcoff , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("XcoffSymbolIterator") . finish () } }
    };
}

impl_818!()