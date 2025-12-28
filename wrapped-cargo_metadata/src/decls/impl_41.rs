macro_rules! deps {
    () => {
        Edition!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl fmt :: Display for Edition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . as_str ()) } }
    };
}

impl_41!()