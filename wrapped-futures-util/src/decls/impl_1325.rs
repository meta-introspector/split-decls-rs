macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_1325 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync > Sync for MutexGuard < '_ , T > { }
    };
}

impl_1325!()