macro_rules! deps {
    () => {
        Protocols!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Protocols < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_33!()