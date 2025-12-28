macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl fmt :: Display for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_24!()