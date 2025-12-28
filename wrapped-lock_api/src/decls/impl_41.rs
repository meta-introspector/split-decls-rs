macro_rules! deps {
    () => {
        RawMutex!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawMutex + 'a , T : ? Sized + Send + 'a > Send for MappedMutexGuard < 'a , R , T > where R :: GuardMarker : Send { }
    };
}

impl_41!()