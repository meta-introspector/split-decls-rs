macro_rules! deps {
    () => {
        OwnedMutexGuard!();
    };
}

macro_rules! impl_1297 {
    () => {
        deps!();
        impl < T : ? Sized + fmt :: Debug > fmt :: Debug for OwnedMutexGuard < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OwnedMutexGuard") . field ("value" , & & * * self) . field ("mutex" , & self . mutex) . finish () } }
    };
}

impl_1297!()