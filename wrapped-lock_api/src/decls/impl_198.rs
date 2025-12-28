macro_rules! deps {
    () => {
        RawRwLock!();
        MappedRwLockWriteGuard!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > DerefMut for MappedRwLockWriteGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . data } } }
    };
}

impl_198!();