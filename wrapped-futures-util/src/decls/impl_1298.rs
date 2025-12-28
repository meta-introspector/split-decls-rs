macro_rules! deps {
    () => {
        OwnedMutexGuard!();
    };
}

macro_rules! impl_1298 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for OwnedMutexGuard < T > { fn drop (& mut self) { self . mutex . unlock () } }
    };
}

impl_1298!();