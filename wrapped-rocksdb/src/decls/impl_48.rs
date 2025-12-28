macro_rules! deps {
    () => {
        ColumnFamily!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Drop for ColumnFamily { fn drop (& mut self) { destroy_handle (self . inner) ; } }
    };
}

impl_48!()