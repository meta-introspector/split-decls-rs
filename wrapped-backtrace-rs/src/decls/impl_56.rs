macro_rules! deps {
    () => {
        Init!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Drop for Init { fn drop (& mut self) { unsafe { let r = ReleaseMutex (self . lock) ; debug_assert ! (r != 0) ; } } }
    };
}

impl_56!()