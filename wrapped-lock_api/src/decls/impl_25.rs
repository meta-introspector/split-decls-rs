macro_rules! deps {
    () => {
        MutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Deref for MutexGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . mutex . data . get () } } }
    };
}

impl_25!();