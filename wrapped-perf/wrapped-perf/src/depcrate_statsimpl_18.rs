// Generated macro for impl_18 (impl)
macro_rules! Depcrate_statsimpl_18 {
() => {
// Module: crate::stats
// Provides: {"impl_18"}
// Dependencies: {}
impl Default for Stats { fn default () -> Self { Self { start_instant : Instant :: now () , start : SystemTime :: now () , upload_duration : Histogram :: new (3) . unwrap () , download_duration : Histogram :: new (3) . unwrap () , fbl : Histogram :: new (3) . unwrap () , upload_throughput : Histogram :: new (3) . unwrap () , download_throughput : Histogram :: new (3) . unwrap () , requests : 0 , intervals : vec ! [] , } } }
};
}
