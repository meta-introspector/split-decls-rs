macro_rules! deps {
    () => {
        UnboundColumnFamily!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Drop for UnboundColumnFamily { fn drop (& mut self) { destroy_handle (self . inner) ; } }
    };
}

impl_50!();