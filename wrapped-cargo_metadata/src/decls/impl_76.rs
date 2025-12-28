macro_rules! deps {
    () => {
        Result!();
        Edition!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl fmt :: Display for Edition { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . as_str ()) } }
    };
}

impl_76!()