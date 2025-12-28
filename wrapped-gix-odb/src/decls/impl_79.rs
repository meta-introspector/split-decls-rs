macro_rules! deps {
    () => {
        Ordering!();
        IncOnNewAndDecOnDrop!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Drop for IncOnNewAndDecOnDrop < '_ > { fn drop (& mut self) { self . 0 . fetch_sub (1 , Ordering :: SeqCst) ; } }
    };
}

impl_79!();