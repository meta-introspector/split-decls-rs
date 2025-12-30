// Generated macro for impl_142 (impl)
macro_rules! Depcrate_dbimpl_142 {
() => {
// Module: crate::db
// Provides: {"impl_142"}
// Dependencies: {}
impl ThreadMode for MultiThreaded { fn new_cf_map_internal (cfs : BTreeMap < String , * mut ffi :: rocksdb_column_family_handle_t > ,) -> Self { Self { cfs : RwLock :: new (cfs . into_iter () . map (| (n , c) | (n , Arc :: new (UnboundColumnFamily { inner : c }))) . collect () ,) , } } fn drop_all_cfs_internal (& mut self) { self . cfs . write () . unwrap () . clear () ; } }
};
}
