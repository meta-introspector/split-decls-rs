macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Display for LexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("cannot parse string into token stream") } }
    };
}

impl_91!();