macro_rules! deps {
    () => {
        Bytes!();
        Result!();
    };
}

macro_rules! impl_1077 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Bytes < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { debug_list_bytes (& self . 0 , f) } }
    };
}

impl_1077!()