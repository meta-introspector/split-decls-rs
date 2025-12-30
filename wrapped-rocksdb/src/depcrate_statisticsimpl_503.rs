// Generated macro for impl_503 (impl)
macro_rules! Depcrate_statisticsimpl_503 {
() => {
// Module: crate::statistics
// Provides: {"impl_503"}
// Dependencies: {}
impl Drop for HistogramData { fn drop (& mut self) { unsafe { ffi :: rocksdb_statistics_histogram_data_destroy (self . inner) ; } } }
};
}
