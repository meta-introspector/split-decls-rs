macro_rules! deps {
    () => {
        RwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > DerefMut for RwLockWriteGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . rwlock . data . get () } } }
    };
}

impl_145!();