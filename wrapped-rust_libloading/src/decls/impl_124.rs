macro_rules! deps {
    () => {
        ErrorModeGuard!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl Drop for ErrorModeGuard { fn drop (& mut self) { unsafe { SetThreadErrorMode (self . 0 , ptr :: null_mut ()) ; } } }
    };
}

impl_124!()