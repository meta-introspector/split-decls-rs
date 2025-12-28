macro_rules! deps {
    () => {
        MacError!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl fmt :: Display for MacError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("MAC tag mismatch") } }
    };
}

impl_16!();