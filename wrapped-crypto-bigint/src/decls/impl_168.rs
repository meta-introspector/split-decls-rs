macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl fmt :: LowerHex for Limb { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "0x") ? ; } write ! (f , "{:0width$x}" , & self . 0 , width = Self :: BYTES * 2) } }
    };
}

impl_168!();