macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Drop for Blob < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . data) ; } }
    };
}

impl_54!()