macro_rules! deps {
    () => {
        Send!();
        BiLockGuard!();
    };
}

macro_rules! impl_1270 {
    () => {
        deps!();
        unsafe impl < T : Send + Sync > Sync for BiLockGuard < '_ , T > { }
    };
}

impl_1270!();