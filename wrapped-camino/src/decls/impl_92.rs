macro_rules! deps {
    () => {
        FromPathError!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl fmt :: Display for FromPathError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Path contains invalid UTF-8") } }
    };
}

impl_92!()