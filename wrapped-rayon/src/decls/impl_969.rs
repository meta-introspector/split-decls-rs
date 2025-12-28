macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_969 {
    () => {
        deps!();
        impl < I : Debug , F > Debug for Update < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Update") . field ("base" , & self . base) . finish () } }
    };
}

impl_969!();