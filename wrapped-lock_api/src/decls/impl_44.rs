macro_rules! deps {
    () => {
        RawMutex!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Deref for MappedMutexGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . data } } }
    };
}

impl_44!();