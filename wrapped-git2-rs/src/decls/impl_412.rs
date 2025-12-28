macro_rules! deps {
    () => {
        Mailmap!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl Drop for Mailmap { fn drop (& mut self) { unsafe { raw :: git_mailmap_free (self . raw) ; } } }
    };
}

impl_412!();