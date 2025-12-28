macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Drop for Entry < '_ > { fn drop (& mut self) { if self . remaining == Some (0) { self . parent . path_buf = self . path_buf . take () ; } } }
    };
}

impl_7!()