macro_rules! deps {
    () => {
        MappedMutexGuard!();
    };
}

macro_rules! impl_1329 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync , U : ? Sized + Sync > Sync for MappedMutexGuard < '_ , T , U > { }
    };
}

impl_1329!()