macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Display for LexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . inner , f) } }
    };
}

impl_31!()