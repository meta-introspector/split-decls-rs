macro_rules! deps {
    () => {
        CachedStatement!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl Drop for CachedStatement < '_ > { # [inline] fn drop (& mut self) { if let Some (stmt) = self . stmt . take () { self . cache . cache_stmt (unsafe { stmt . into_raw () }) ; } } }
    };
}

impl_66!()