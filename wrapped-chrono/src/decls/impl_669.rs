macro_rules! deps {
    () => {
        Utc!();
    };
}

macro_rules! impl_669 {
    () => {
        deps!();
        impl fmt :: Display for Utc { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "UTC") } }
    };
}

impl_669!();