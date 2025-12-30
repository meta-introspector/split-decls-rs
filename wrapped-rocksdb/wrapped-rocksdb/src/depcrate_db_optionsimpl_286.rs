// Generated macro for impl_286 (impl)
macro_rules! Depcrate_db_optionsimpl_286 {
() => {
// Module: crate::db_options
// Provides: {"impl_286"}
// Dependencies: {}
impl Drop for UniversalCompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_universal_compaction_options_destroy (self . inner) ; } } }
};
}
