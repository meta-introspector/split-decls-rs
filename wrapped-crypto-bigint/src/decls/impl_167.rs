macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl fmt :: Binary for Limb { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "0b") ? ; } write ! (f , "{:0width$b}" , & self . 0 , width = Self :: BITS as usize) } }
    };
}

impl_167!()