macro_rules! deps {
    () => {
        GlobError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Display for GlobError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "attempting to read `{}` resulted in an error: {}" , self . path . display () , self . error) } }
    };
}

impl_9!();