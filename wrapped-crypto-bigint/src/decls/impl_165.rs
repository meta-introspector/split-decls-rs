macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl fmt :: Debug for Limb { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Limb(0x{self:X})") } }
    };
}

impl_165!()