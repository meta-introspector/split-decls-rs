macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < 'a , T : ? Sized + 'a > Drop for MutexGuard < 'a , T > { fn drop (& mut self) { self . data = None ; self . lock . object . release_lock () ; } }
    };
}

impl_304!();