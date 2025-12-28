macro_rules! deps {
    () => {
        SelectTimeoutError!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl fmt :: Display for SelectTimeoutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "timed out waiting on select" . fmt (f) } }
    };
}

impl_96!()