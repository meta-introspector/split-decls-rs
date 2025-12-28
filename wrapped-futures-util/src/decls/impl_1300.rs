macro_rules! deps {
    () => {
        OwnedMutexGuard!();
    };
}

macro_rules! impl_1300 {
    () => {
        deps!();
        impl < T : ? Sized > DerefMut for OwnedMutexGuard < T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . value . get () } } }
    };
}

impl_1300!()