macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Drop for Library { fn drop (& mut self) { unsafe { dlclose (self . handle) ; } } }
    };
}

impl_93!();