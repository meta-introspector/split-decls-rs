macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Drop for Tag < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . data) ; } }
    };
}

impl_58!()