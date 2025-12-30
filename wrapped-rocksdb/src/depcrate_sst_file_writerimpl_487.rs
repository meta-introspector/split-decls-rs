// Generated macro for impl_487 (impl)
macro_rules! Depcrate_sst_file_writerimpl_487 {
() => {
// Module: crate::sst_file_writer
// Provides: {"impl_487"}
// Dependencies: {}
impl Drop for EnvOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_envoptions_destroy (self . inner) ; } } }
};
}
