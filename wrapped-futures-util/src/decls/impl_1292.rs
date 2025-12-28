macro_rules! deps {
    () => {
        OwnedMutexLockFuture!();
    };
}

macro_rules! impl_1292 {
    () => {
        deps!();
        impl < T : ? Sized > fmt :: Debug for OwnedMutexLockFuture < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OwnedMutexLockFuture") . field ("was_acquired" , & self . mutex . is_none ()) . field ("mutex" , & self . mutex) . field ("wait_key" , & (if self . wait_key == WAIT_KEY_NONE { None } else { Some (self . wait_key) }) ,) . finish () } }
    };
}

impl_1292!();