macro_rules! deps {
    () => {
        Aborted!();
    };
}

macro_rules! impl_1341 {
    () => {
        deps!();
        impl fmt :: Display for Aborted { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "`Abortable` future has been aborted") } }
    };
}

impl_1341!()