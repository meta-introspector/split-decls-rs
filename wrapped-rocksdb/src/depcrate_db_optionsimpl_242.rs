// Generated macro for impl_242 (impl)
macro_rules! Depcrate_db_optionsimpl_242 {
() => {
// Module: crate::db_options
// Provides: {"impl_242"}
// Dependencies: {}
impl Clone for Options { fn clone (& self) -> Self { let inner = unsafe { ffi :: rocksdb_options_create_copy (self . inner) } ; assert ! (! inner . is_null () , "Could not copy RocksDB options") ; Self { inner , outlive : self . outlive . clone () , } } }
};
}
