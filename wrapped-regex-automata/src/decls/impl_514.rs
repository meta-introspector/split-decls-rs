macro_rules! deps {
    () => {
        NFA!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl fmt :: Debug for NFA { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_514!()