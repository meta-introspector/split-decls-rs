macro_rules! deps {
    () => {
        AsColumnFamilyRef!();
        BoundColumnFamily!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl AsColumnFamilyRef for Arc < BoundColumnFamily < '_ > > { fn inner (& self) -> * mut ffi :: rocksdb_column_family_handle_t { self . inner } }
    };
}

impl_56!();