macro_rules! deps {
    () => {
        MappedMutexGuard!();
    };
}

macro_rules! impl_1314 {
    () => {
        deps!();
        impl < T : ? Sized , U : ? Sized + fmt :: Debug > fmt :: Debug for MappedMutexGuard < '_ , T , U > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MappedMutexGuard") . field ("value" , & & * * self) . field ("mutex" , & self . mutex) . finish () } }
    };
}

impl_1314!()