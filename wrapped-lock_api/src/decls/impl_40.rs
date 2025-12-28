macro_rules! deps {
    () => {
        MappedMutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawMutex + Sync + 'a , T : ? Sized + Sync + 'a > Sync for MappedMutexGuard < 'a , R , T > { }
    };
}

impl_40!()