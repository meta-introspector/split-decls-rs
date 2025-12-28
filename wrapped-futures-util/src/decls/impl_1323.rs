macro_rules! deps {
    () => {
        OwnedMutexLockFuture!();
    };
}

macro_rules! impl_1323 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > Sync for OwnedMutexLockFuture < T > { }
    };
}

impl_1323!()