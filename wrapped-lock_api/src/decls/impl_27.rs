macro_rules! deps {
    () => {
        RawMutex!();
        MutexGuard!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > Drop for MutexGuard < 'a , R , T > { # [inline] fn drop (& mut self) { unsafe { self . mutex . raw . unlock () ; } } }
    };
}

impl_27!();