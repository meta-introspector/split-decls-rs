macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl Drop for Buffer < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . inner) ; } }
    };
}

impl_318!()