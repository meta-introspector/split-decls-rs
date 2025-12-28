macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! impl_765 {
    () => {
        deps!();
        impl < I : Debug , P > Debug for Positions < I , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Positions") . field ("base" , & self . base) . finish () } }
    };
}

impl_765!()