// Generated macro for impl_173 (impl)
macro_rules! Depcrate_db_iteratorimpl_173 {
() => {
// Module: crate::db_iterator
// Provides: {"impl_173"}
// Dependencies: {}
impl < D : DBAccess > Drop for DBRawIteratorWithThreadMode < '_ , D > { fn drop (& mut self) { unsafe { ffi :: rocksdb_iter_destroy (self . inner . as_ptr ()) ; } } }
};
}
