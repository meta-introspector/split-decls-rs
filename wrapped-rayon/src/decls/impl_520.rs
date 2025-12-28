macro_rules! deps {
    () => {
        FlatMapIter!();
    };
}

macro_rules! impl_520 {
    () => {
        deps!();
        impl < I : Debug , F > Debug for FlatMapIter < I , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FlatMapIter") . field ("base" , & self . base) . finish () } }
    };
}

impl_520!();