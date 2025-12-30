// Generated macro for impl_312 (impl)
macro_rules! Depcrate_db_pinnable_sliceimpl_312 {
() => {
// Module: crate::db_pinnable_slice
// Provides: {"impl_312"}
// Dependencies: {}
impl Drop for DBPinnableSlice < '_ > { fn drop (& mut self) { unsafe { ffi :: rocksdb_pinnableslice_destroy (self . ptr) ; } } }
};
}
