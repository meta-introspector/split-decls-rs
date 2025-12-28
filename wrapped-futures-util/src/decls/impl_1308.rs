macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_1308 {
    () => {
        deps!();
        impl < T : ? Sized + fmt :: Debug > fmt :: Debug for MutexGuard < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MutexGuard") . field ("value" , & & * * self) . field ("mutex" , & self . mutex) . finish () } }
    };
}

impl_1308!();