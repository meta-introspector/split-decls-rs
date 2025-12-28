macro_rules! deps {
    () => {
        Hash128!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Debug for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_10!()