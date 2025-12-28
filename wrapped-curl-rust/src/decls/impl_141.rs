macro_rules! deps {
    () => {
        DetachGuard!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Drop for DetachGuard { fn drop (& mut self) { let _ = self . detach () ; } }
    };
}

impl_141!()