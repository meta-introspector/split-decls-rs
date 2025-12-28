macro_rules! deps {
    () => {
        BiLockGuard!();
    };
}

macro_rules! impl_1274 {
    () => {
        deps!();
        impl < T > Drop for BiLockGuard < '_ , T > { fn drop (& mut self) { self . bilock . unlock () ; } }
    };
}

impl_1274!()