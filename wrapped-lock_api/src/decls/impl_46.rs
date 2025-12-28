macro_rules! deps {
    () => {
        MappedMutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Drop for MappedMutexGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . raw . unlock () ; } } }
    };
}

impl_46!();