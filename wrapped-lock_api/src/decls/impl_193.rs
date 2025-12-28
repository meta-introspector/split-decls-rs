macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockWriteGuard!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + Sync + 'a > Sync for MappedRwLockWriteGuard < 'a , R , T > { }
    };
}

impl_193!();