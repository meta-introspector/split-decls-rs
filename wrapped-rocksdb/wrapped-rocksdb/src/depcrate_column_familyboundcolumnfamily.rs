// Generated macro for BoundColumnFamily (struct)
macro_rules! Depcrate_column_familyBoundColumnFamily {
() => {
// Module: crate::column_family
// Provides: {"BoundColumnFamily"}
// Dependencies: {}
# [doc = " A specialized opaque type used to represent a column family by the [`MultiThreaded`]"] # [doc = " mode. Clone (and Copy) is derived to behave like `&ColumnFamily` (this is used for"] # [doc = " single-threaded mode). `Clone`/`Copy` is safe because this lifetime is bound to DB like"] # [doc = " iterators/snapshots. On top of it, this is as cheap and small as `&ColumnFamily` because"] # [doc = " this only has a single pointer-wide field."] pub struct BoundColumnFamily < 'a > { pub (crate) inner : * mut ffi :: rocksdb_column_family_handle_t , pub (crate) multi_threaded_cfs : std :: marker :: PhantomData < & 'a MultiThreaded > , }
};
}
