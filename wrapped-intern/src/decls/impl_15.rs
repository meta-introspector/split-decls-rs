macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl fmt :: Debug for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_str () . fmt (f) } }
    };
}

impl_15!()