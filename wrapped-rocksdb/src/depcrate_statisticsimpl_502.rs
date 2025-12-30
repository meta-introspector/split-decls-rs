// Generated macro for impl_502 (impl)
macro_rules! Depcrate_statisticsimpl_502 {
() => {
// Module: crate::statistics
// Provides: {"impl_502"}
// Dependencies: {}
impl Default for HistogramData { fn default () -> Self { let histogram_data_inner = unsafe { ffi :: rocksdb_statistics_histogram_data_create () } ; assert ! (! histogram_data_inner . is_null () , "Could not create RocksDB histogram data") ; Self { inner : histogram_data_inner , } } }
};
}
