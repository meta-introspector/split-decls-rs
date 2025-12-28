macro_rules! deps {
    () => {
        MappedMutexGuard!();
    };
}

macro_rules! impl_1317 {
    () => {
        deps!();
        impl < T : ? Sized , U : ? Sized > DerefMut for MappedMutexGuard < '_ , T , U > { fn deref_mut (& mut self) -> & mut U { unsafe { & mut * self . value } } }
    };
}

impl_1317!();