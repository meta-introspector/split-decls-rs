macro_rules! deps {
    () => {
        ErrorImpl!();
        Result!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl Debug for ErrorImpl { fn fmt (& self , _formatter : & mut fmt :: Formatter) -> fmt :: Result { unimplemented ! () } }
    };
}

impl_89!()