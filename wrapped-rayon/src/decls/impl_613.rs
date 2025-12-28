macro_rules! deps {
    () => {
        Inspect!();
    };
}

macro_rules! impl_613 {
    () => {
        deps!();
        impl < I : Debug , F > Debug for Inspect < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Inspect") . field ("base" , & self . base) . finish () } }
    };
}

impl_613!();