// Generated macro for impl_490 (impl)
macro_rules! Depcrate_sst_file_writerimpl_490 {
() => {
// Module: crate::sst_file_writer
// Provides: {"impl_490"}
// Dependencies: {}
impl Drop for SstFileWriter < '_ > { fn drop (& mut self) { unsafe { ffi :: rocksdb_sstfilewriter_destroy (self . inner) ; } } }
};
}
