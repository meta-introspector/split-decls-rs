macro_rules! deps {
    () => {
        OwnedMutexGuard!();
    };
}

macro_rules! impl_1327 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync > Sync for OwnedMutexGuard < T > { }
    };
}

impl_1327!()