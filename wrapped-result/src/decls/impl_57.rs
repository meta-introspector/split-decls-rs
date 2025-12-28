macro_rules! deps {
    () => {
        BasicString!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Drop for BasicString { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { SysFreeString (self . 0) } } } }
    };
}

impl_57!();