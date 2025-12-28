macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Drop for Commit < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . data) ; } }
    };
}

impl_60!();