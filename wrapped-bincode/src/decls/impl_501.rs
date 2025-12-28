macro_rules! deps {
    () => {
        EncodeError!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl core :: fmt :: Display for EncodeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:?}" , self) } }
    };
}

impl_501!();