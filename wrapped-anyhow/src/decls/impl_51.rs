macro_rules! deps {
    () => {
        ErrorImpl!();
        Error!();
        Result!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { unsafe { ErrorImpl :: debug (self . inner . by_ref () , formatter) } } }
    };
}

impl_51!();