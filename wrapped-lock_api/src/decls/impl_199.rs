macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockWriteGuard!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for MappedRwLockWriteGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock_exclusive () ; } } }
    };
}

impl_199!();