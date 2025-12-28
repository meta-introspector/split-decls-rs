macro_rules! deps {
    () => {
        FilePathToUriError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for FilePathToUriError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "The given file path could not be converted to a URI") } }
    };
}

impl_7!();