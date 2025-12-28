macro_rules! deps {
    () => {
        BoundColumnFamily!();
        ColumnFamily!();
    };
}

macro_rules! AsColumnFamilyRef {
    () => {
        deps!();
        # [doc = " Utility trait to accept both supported references to `ColumnFamily`"] # [doc = " (`&ColumnFamily` and `BoundColumnFamily`)"] pub trait AsColumnFamilyRef { fn inner (& self) -> * mut ffi :: rocksdb_column_family_handle_t ; }
    };
}

AsColumnFamilyRef!();