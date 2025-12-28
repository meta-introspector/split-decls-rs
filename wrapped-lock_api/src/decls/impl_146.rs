macro_rules! deps {
    () => {
        RwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for RwLockWriteGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_exclusive () ; } } }
    };
}

impl_146!()