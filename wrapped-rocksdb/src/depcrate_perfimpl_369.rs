// Generated macro for impl_369 (impl)
macro_rules! Depcrate_perfimpl_369 {
() => {
// Module: crate::perf
// Provides: {"impl_369"}
// Dependencies: {}
impl Drop for MemoryUsageBuilder { fn drop (& mut self) { unsafe { ffi :: rocksdb_memory_consumers_destroy (self . inner) ; } } }
};
}
