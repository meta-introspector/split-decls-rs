macro_rules! deps {
    () => {
        RwLockReadGuard!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl < 'a , T : 'a > Drop for RwLockReadGuard < 'a , T > { fn drop (& mut self) { self . data = None ; self . lock . object . release_read_lock () } }
    };
}

impl_317!()