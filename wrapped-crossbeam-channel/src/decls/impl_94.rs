macro_rules! deps {
    () => {
        TrySelectError!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl fmt :: Display for TrySelectError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "all operations in select would block" . fmt (f) } }
    };
}

impl_94!();