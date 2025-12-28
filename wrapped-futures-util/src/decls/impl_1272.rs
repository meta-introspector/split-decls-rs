macro_rules! deps {
    () => {
        BiLockGuard!();
    };
}

macro_rules! impl_1272 {
    () => {
        deps!();
        impl < T : Unpin > DerefMut for BiLockGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . bilock . arc . value . as_ref () . unwrap () . get () } } }
    };
}

impl_1272!();