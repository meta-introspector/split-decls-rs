macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockReadGuard!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for RwLockReadGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_shared () ; } } }
    };
}

impl_127!();