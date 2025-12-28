macro_rules! deps {
    () => {
        Cred!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl Drop for Cred { fn drop (& mut self) { if ! self . raw . is_null () { unsafe { if let Some (f) = (* self . raw) . free { f (self . raw) } } } } }
    };
}

impl_283!()