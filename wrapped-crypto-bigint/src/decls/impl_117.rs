macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: Debug for Int < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Int(0x{self:X})") } }
    };
}

impl_117!()