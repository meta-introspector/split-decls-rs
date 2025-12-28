macro_rules! deps {
    () => {
        MappedRwLockReadGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for MappedRwLockReadGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
    };
}

impl_187!()