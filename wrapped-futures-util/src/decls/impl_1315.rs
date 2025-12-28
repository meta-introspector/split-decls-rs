macro_rules! deps {
    () => {
        MappedMutexGuard!();
    };
}

macro_rules! impl_1315 {
    () => {
        deps!();
        impl < T : ? Sized , U : ? Sized > Drop for MappedMutexGuard < '_ , T , U > { fn drop (& mut self) { self . mutex . unlock () } }
    };
}

impl_1315!();