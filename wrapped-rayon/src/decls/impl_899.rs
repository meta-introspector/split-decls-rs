macro_rules! deps {
    () => {
        ParallelIterator!();
        TryFold!();
    };
}

macro_rules! impl_899 {
    () => {
        deps!();
        impl < U , I : ParallelIterator + Debug , ID , F > Debug for TryFold < I , U , ID , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TryFold") . field ("base" , & self . base) . finish () } }
    };
}

impl_899!();