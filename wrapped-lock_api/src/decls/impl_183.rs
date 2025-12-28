macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockReadGuard!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + Sync + 'a > Sync for MappedRwLockReadGuard < 'a , R , T > { }
    };
}

impl_183!();