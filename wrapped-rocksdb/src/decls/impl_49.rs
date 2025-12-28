macro_rules! deps {
    () => {
        BoundColumnFamily!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl Drop for BoundColumnFamily < '_ > { fn drop (& mut self) { destroy_handle (self . inner) ; } }
    };
}

impl_49!();