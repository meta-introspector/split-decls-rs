macro_rules! deps {
    () => {
        Canceled!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl fmt :: Display for Canceled { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "oneshot canceled") } }
    };
}

impl_115!()