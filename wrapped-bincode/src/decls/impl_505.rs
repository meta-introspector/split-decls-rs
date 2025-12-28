macro_rules! deps {
    () => {
        DecodeError!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        impl core :: fmt :: Display for DecodeError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{:?}" , self) } }
    };
}

impl_505!();