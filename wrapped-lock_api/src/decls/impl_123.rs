macro_rules! deps {
    () => {
        RwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        unsafe impl < R : RawRwLock + Sync , T : Sync + ? Sized > Sync for RwLockReadGuard < '_ , R , T > { }
    };
}

impl_123!();