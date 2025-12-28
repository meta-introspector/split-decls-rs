macro_rules! deps {
    () => {
        MappedRwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Drop for MappedRwLockReadGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock_shared () ; } } }
    };
}

impl_188!();