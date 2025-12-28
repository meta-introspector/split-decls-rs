macro_rules! deps {
    () => {
        Literal!();
        Result!();
        Formatter!();
        Bytes!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Literal { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { crate :: debug :: Bytes (& self . 0) . fmt (f) } }
    };
}

impl_226!()