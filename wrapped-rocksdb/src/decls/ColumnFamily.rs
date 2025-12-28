macro_rules! ColumnFamily {
    () => {
        # [doc = " An opaque type used to represent a column family. Returned from some functions, and used"] # [doc = " in others"] pub struct ColumnFamily { pub (crate) inner : * mut ffi :: rocksdb_column_family_handle_t , }
    };
}

ColumnFamily!()