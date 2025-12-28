macro_rules! deps {
    () => {
        Send!();
        OwnedMutexLockFuture!();
    };
}

macro_rules! impl_1322 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for OwnedMutexLockFuture < T > { }
    };
}

impl_1322!();