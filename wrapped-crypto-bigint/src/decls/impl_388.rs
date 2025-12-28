macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: Debug for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Uint(0x{:X})" , self . as_uint_ref ()) } }
    };
}

impl_388!()