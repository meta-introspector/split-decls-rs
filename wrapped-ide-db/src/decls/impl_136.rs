macro_rules! deps {
    () => {
        Result!();
        RenameError!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl fmt :: Display for RenameError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_136!()