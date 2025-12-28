macro_rules! deps {
    () => {
        Send!();
        MappedMutexGuard!();
    };
}

macro_rules! impl_1328 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Send , U : ? Sized + Send > Send for MappedMutexGuard < '_ , T , U > { }
    };
}

impl_1328!()