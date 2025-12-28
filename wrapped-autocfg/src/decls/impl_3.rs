macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match self . kind { ErrorKind :: Io (ref e) => e . fmt (f) , ErrorKind :: Num (ref e) => e . fmt (f) , ErrorKind :: Utf8 (ref e) => e . fmt (f) , ErrorKind :: Process (ref status) => { write ! (f , "process exited unsuccessfully: {}" , status) } ErrorKind :: Other (s) => s . fmt (f) , } } }
    };
}

impl_3!()