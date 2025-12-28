macro_rules! deps {
    () => {
        MutexLockFuture!();
    };
}

macro_rules! impl_1321 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > Sync for MutexLockFuture < '_ , T > { }
    };
}

impl_1321!();