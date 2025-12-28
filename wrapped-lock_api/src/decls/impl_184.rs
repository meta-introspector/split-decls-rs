macro_rules! deps {
    () => {
        MappedRwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + Sync + 'a > Send for MappedRwLockReadGuard < 'a , R , T > where R :: GuardMarker : Send { }
    };
}

impl_184!();