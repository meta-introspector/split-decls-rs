macro_rules! deps {
    () => {
        RawMutex!();
        MutexGuard!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > DerefMut for MutexGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . data . get () } } }
    };
}

impl_26!();