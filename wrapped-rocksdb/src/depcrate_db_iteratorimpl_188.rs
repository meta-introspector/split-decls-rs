// Generated macro for impl_188 (impl)
macro_rules! Depcrate_db_iteratorimpl_188 {
() => {
// Module: crate::db_iterator
// Provides: {"impl_188"}
// Dependencies: {}
impl Drop for DBWALIterator { fn drop (& mut self) { unsafe { ffi :: rocksdb_wal_iter_destroy (self . inner) ; } } }
};
}
