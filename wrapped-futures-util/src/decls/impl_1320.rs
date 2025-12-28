macro_rules! deps {
    () => {
        MutexLockFuture!();
        Send!();
    };
}

macro_rules! impl_1320 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for MutexLockFuture < '_ , T > { }
    };
}

impl_1320!();