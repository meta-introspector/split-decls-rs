macro_rules! deps {
    () => {
        Utf8Component!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl fmt :: Display for Utf8Component < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self . as_str () , f) } }
    };
}

impl_59!();