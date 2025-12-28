macro_rules! deps {
    () => {
        ProcessError!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl fmt :: Display for ProcessError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . desc . fmt (f) } }
    };
}

impl_63!()