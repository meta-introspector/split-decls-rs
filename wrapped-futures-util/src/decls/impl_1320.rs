macro_rules! deps {
    () => {
        Send!();
        MutexLockFuture!();
    };
}

macro_rules! impl_1320 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for MutexLockFuture < '_ , T > { }
    };
}

impl_1320!()