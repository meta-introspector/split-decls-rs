// Generated macro for AsColumnFamilyRef (trait)
macro_rules! Depcrate_column_familyAsColumnFamilyRef {
() => {
// Module: crate::column_family
// Provides: {"AsColumnFamilyRef"}
// Dependencies: {}
# [doc = " Utility trait to accept both supported references to `ColumnFamily`"] # [doc = " (`&ColumnFamily` and `BoundColumnFamily`)"] pub trait AsColumnFamilyRef { fn inner (& self) -> * mut ffi :: rocksdb_column_family_handle_t ; }
};
}
