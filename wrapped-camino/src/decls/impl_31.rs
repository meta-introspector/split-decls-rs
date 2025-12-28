macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8Path { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self . as_str () , f) } }
    };
}

impl_31!();