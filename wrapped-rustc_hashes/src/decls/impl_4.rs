macro_rules! deps {
    () => {
        Hash64!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Debug for Hash64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_4!();