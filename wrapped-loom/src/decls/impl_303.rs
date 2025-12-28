macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > ops :: DerefMut for MutexGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { self . data . as_mut () . unwrap () . deref_mut () } }
    };
}

impl_303!();