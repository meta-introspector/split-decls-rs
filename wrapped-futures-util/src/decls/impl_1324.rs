macro_rules! deps {
    () => {
        MutexGuard!();
        Send!();
    };
}

macro_rules! impl_1324 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send > Send for MutexGuard < '_ , T > { }
    };
}

impl_1324!()