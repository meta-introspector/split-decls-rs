macro_rules! deps {
    () => {
        RwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        unsafe impl < R : RawRwLock + Sync , T : Sync + ? Sized > Sync for RwLockWriteGuard < '_ , R , T > { }
    };
}

impl_139!()