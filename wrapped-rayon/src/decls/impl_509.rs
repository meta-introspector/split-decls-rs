macro_rules! deps {
    () => {
        FlatMap!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < I : Debug , F > Debug for FlatMap < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatMap") . field ("base" , & self . base) . finish () } }
    };
}

impl_509!();