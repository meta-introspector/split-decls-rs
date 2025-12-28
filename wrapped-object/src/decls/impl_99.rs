macro_rules! deps {
    () => {
        Bytes!();
        Result!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'data > fmt :: Debug for Bytes < 'data > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { debug_list_bytes (self . 0 , fmt) } }
    };
}

impl_99!()