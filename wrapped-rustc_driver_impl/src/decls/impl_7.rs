macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Utf8Error (path) => write ! (fmt , "UTF-8 error in {path}") , Error :: IOError (path , err) => write ! (fmt , "IO error: {path}: {err}") , Error :: ShellParseError (path) => write ! (fmt , "invalid shell-style arguments in {path}") , } } }
    };
}

impl_7!()