macro_rules! deps {
    () => {
        MappedRwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for MappedRwLockWriteGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
    };
}

impl_197!();