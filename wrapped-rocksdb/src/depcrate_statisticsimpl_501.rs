// Generated macro for impl_501 (impl)
macro_rules! Depcrate_statisticsimpl_501 {
() => {
// Module: crate::statistics
// Provides: {"impl_501"}
// Dependencies: {}
impl HistogramData { pub fn new () -> HistogramData { HistogramData :: default () } pub fn median (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_median (self . inner) } } pub fn average (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_average (self . inner) } } pub fn p95 (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_p95 (self . inner) } } pub fn p99 (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_p99 (self . inner) } } pub fn max (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_max (self . inner) } } pub fn min (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_min (self . inner) } } pub fn sum (& self) -> u64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_sum (self . inner) } } pub fn count (& self) -> u64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_count (self . inner) } } pub fn std_dev (& self) -> f64 { unsafe { ffi :: rocksdb_statistics_histogram_data_get_std_dev (self . inner) } } }
};
}
