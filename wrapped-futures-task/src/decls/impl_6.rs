macro_rules! deps {
    () => {
        SpawnError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl fmt :: Display for SpawnError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Executor is shutdown") } }
    };
}

impl_6!();