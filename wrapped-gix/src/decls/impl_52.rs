macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl Drop for Object < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . data) ; } }
    };
}

impl_52!()