macro_rules! deps {
    () => {
        Utf8Components!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8Components < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_41!()