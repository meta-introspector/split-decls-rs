macro_rules! deps {
    () => {
        MappedMutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawMutex + 'a , T : ? Sized + Send + 'a > Send for MappedMutexGuard < 'a , R , T > where R :: GuardMarker : Send { }
    };
}

impl_41!();