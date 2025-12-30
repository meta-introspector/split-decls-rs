// Generated macro for impl_200 (impl)
macro_rules! Depcrate_db_optionsimpl_200 {
() => {
// Module: crate::db_options
// Provides: {"impl_200"}
// Dependencies: {}
impl Drop for WriteBufferManagerWrapper { fn drop (& mut self) { unsafe { ffi :: rocksdb_write_buffer_manager_destroy (self . inner . as_ptr ()) ; } } }
};
}
