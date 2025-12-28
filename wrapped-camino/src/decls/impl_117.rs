macro_rules! deps {
    () => {
        FromOsStrError!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl fmt :: Display for FromOsStrError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "OsStr contains invalid UTF-8") } }
    };
}

impl_117!();