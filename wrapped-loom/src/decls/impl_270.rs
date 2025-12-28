macro_rules! deps {
    () => {
        AtomicPtr!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < T > std :: fmt :: Debug for AtomicPtr < T > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_270!();