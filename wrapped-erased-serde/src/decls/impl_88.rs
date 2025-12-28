macro_rules! deps {
    () => {
        ErrorImpl!();
        Result!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl Display for ErrorImpl { fn fmt (& self , _formatter : & mut fmt :: Formatter) -> fmt :: Result { unimplemented ! () } }
    };
}

impl_88!()