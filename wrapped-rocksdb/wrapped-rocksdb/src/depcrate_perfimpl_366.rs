// Generated macro for impl_366 (impl)
macro_rules! Depcrate_perfimpl_366 {
() => {
// Module: crate::perf
// Provides: {"impl_366"}
// Dependencies: {}
impl Drop for MemoryUsage { fn drop (& mut self) { unsafe { ffi :: rocksdb_approximate_memory_usage_destroy (self . inner) ; } } }
};
}
