macro_rules! deps {
    () => {
        LengthError!();
        GenericArray!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl core :: fmt :: Display for LengthError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("LengthError: Slice or iterator does not match GenericArray length") } }
    };
}

impl_195!()