macro_rules! deps {
    () => {
        AllocError!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl fmt :: Display for AllocError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("memory allocation failed") } }
    };
}

impl_11!()