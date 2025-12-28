macro_rules! deps {
    () => {
        MutexLockFuture!();
    };
}

macro_rules! impl_1302 {
    () => {
        deps!();
        impl < T : ? Sized > fmt :: Debug for MutexLockFuture < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("MutexLockFuture") . field ("was_acquired" , & self . mutex . is_none ()) . field ("mutex" , & self . mutex) . field ("wait_key" , & (if self . wait_key == WAIT_KEY_NONE { None } else { Some (self . wait_key) }) ,) . finish () } }
    };
}

impl_1302!();