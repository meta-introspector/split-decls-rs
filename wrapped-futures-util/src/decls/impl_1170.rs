macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! impl_1170 {
    () => {
        deps!();
        impl Drop for Guard < '_ > { fn drop (& mut self) { unsafe { self . buf . set_len (self . len) ; } } }
    };
}

impl_1170!()