macro_rules! deps {
    () => {
        EasyData!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl fmt :: Debug for EasyData { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { "callbacks ..." . fmt (f) } }
    };
}

impl_52!();