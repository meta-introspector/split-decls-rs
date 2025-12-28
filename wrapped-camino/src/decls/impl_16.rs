macro_rules! deps {
    () => {
        Utf8Ancestors!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8Ancestors < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_16!()