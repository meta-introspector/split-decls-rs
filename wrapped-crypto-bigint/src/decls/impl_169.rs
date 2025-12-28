macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl fmt :: UpperHex for Limb { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{:0width$X}" , & self . 0 , width = Self :: BYTES * 2) } }
    };
}

impl_169!();