macro_rules! deps {
    () => {
        AsColumnFamilyRef!();
        ColumnFamily!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl AsColumnFamilyRef for ColumnFamily { fn inner (& self) -> * mut ffi :: rocksdb_column_family_handle_t { self . inner } }
    };
}

impl_54!();