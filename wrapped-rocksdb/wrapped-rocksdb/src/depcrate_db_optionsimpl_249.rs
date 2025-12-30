// Generated macro for impl_249 (impl)
macro_rules! Depcrate_db_optionsimpl_249 {
() => {
// Module: crate::db_options
// Provides: {"impl_249"}
// Dependencies: {}
impl Drop for IngestExternalFileOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_ingestexternalfileoptions_destroy (self . inner) ; } } }
};
}
