macro_rules! deps {
    () => {
        Result!();
        Error!();
        ErrorImpl!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { ErrorImpl :: display (self . inner . by_ref () , formatter) } } }
    };
}

impl_50!()