macro_rules! deps {
    () => {
        Result!();
        ServerError!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Display for ServerError { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { f . write_str (& self . message) } }
    };
}

impl_32!();