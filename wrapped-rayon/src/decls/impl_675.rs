macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        impl < I : Debug , F > Debug for Map < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Map") . field ("base" , & self . base) . finish () } }
    };
}

impl_675!();