macro_rules! deps {
    () => {
        Send!();
        OwnedMutexGuard!();
    };
}

macro_rules! impl_1326 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for OwnedMutexGuard < T > { }
    };
}

impl_1326!();