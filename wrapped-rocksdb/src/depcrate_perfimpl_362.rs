// Generated macro for impl_362 (impl)
macro_rules! Depcrate_perfimpl_362 {
() => {
// Module: crate::perf
// Provides: {"impl_362"}
// Dependencies: {}
impl Drop for PerfContext { fn drop (& mut self) { unsafe { ffi :: rocksdb_perfcontext_destroy (self . inner) ; } } }
};
}
