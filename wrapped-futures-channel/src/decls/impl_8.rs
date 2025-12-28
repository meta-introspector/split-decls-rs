macro_rules! deps {
    () => {
        TryLock!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T > DerefMut for TryLock < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . __ptr . data . get () } } }
    };
}

impl_8!();