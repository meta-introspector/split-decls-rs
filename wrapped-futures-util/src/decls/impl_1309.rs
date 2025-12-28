macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_1309 {
    () => {
        deps!();
        impl < T : ? Sized > Drop for MutexGuard < '_ , T > { fn drop (& mut self) { self . mutex . unlock () } }
    };
}

impl_1309!()