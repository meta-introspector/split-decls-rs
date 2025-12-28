macro_rules! deps {
    () => {
        RwLockWriteGuard!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < 'a , T : 'a > Drop for RwLockWriteGuard < 'a , T > { fn drop (& mut self) { self . data = None ; self . lock . object . release_write_lock () } }
    };
}

impl_320!();