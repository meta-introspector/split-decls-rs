macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Display for MemoryUsage { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . allocated . fmt (f) } }
    };
}

impl_9!()