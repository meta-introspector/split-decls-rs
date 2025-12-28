macro_rules! deps {
    () => {
        Library!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Drop for Library { fn drop (& mut self) { unsafe { FreeLibrary (self . 0) ; } } }
    };
}

impl_112!();