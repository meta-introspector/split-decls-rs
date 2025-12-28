macro_rules! deps {
    () => {
        Utc!();
    };
}

macro_rules! impl_668 {
    () => {
        deps!();
        impl fmt :: Debug for Utc { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Z") } }
    };
}

impl_668!();