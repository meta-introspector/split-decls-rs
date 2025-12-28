macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Display for Utf8Path { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_13!()