macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl fmt :: Debug for Library { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_149!()