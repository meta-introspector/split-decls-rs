macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_699 {
    () => {
        deps!();
        impl Drop for Repository { fn drop (& mut self) { unsafe { raw :: git_repository_free (self . raw) } } }
    };
}

impl_699!();