macro_rules! deps {
    () => {
        Changeset!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl Drop for Changeset { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3_free (self . cs) ; } } }
    };
}

impl_252!();