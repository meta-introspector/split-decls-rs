macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl Debug for LexError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , f) } }
    };
}

impl_200!()