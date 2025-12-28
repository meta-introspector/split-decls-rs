macro_rules! deps {
    () => {
        MutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        unsafe impl < 'a , R : RawMutex + Sync + 'a , T : ? Sized + Sync + 'a > Sync for MutexGuard < 'a , R , T > { }
    };
}

impl_22!()