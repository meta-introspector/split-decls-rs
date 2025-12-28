macro_rules! deps {
    () => {
        EnterError!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl fmt :: Display for EnterError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "an execution scope has already been entered") } }
    };
}

impl_56!()