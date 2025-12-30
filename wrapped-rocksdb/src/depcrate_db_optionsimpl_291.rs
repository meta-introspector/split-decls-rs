// Generated macro for impl_291 (impl)
macro_rules! Depcrate_db_optionsimpl_291 {
() => {
// Module: crate::db_options
// Provides: {"impl_291"}
// Dependencies: {}
impl Drop for CompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_compactoptions_destroy (self . inner) ; } } }
};
}
