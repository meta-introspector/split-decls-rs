macro_rules! deps {
    () => {
        MappedMutexGuard!();
    };
}

macro_rules! impl_1316 {
    () => {
        deps!();
        impl < T : ? Sized , U : ? Sized > Deref for MappedMutexGuard < '_ , T , U > { type Target = U ; fn deref (& self) -> & U { unsafe { & * self . value } } }
    };
}

impl_1316!();