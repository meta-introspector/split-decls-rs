macro_rules! deps {
    () => {
        RawMutex!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > DerefMut for MappedMutexGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . data } } }
    };
}

impl_45!();