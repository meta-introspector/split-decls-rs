macro_rules! deps {
    () => {
        AlignFromBytesError!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl fmt :: Debug for AlignFromBytesError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
    };
}

impl_40!()