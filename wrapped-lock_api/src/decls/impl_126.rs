macro_rules! deps {
    () => {
        RawRwLock!();
        RwLockReadGuard!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a , R : RawRwLock + 'a , T : ? Sized + 'a > Deref for RwLockReadGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
    };
}

impl_126!();