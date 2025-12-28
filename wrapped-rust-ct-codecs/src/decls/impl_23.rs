macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Overflow => write ! (f , "Output buffer too small or calculation overflow") , Error :: InvalidInput => write ! (f , "Invalid input for the given encoding") , } } }
    };
}

impl_23!();