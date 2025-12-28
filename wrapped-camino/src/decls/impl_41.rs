macro_rules! deps {
    () => {
        Utf8Component!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl fmt :: Debug for Utf8Component < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self . as_os_str () , f) } }
    };
}

impl_41!()