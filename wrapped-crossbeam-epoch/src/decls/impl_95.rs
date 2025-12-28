macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Drop for Guard { # [inline] fn drop (& mut self) { if let Some (local) = unsafe { self . local . as_ref () } { local . unpin () ; } } }
    };
}

impl_95!();