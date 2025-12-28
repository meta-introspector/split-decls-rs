macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Drop for Tree < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . data) ; } }
    };
}

impl_56!();