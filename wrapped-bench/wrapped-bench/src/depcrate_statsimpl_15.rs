// Generated macro for impl_15 (impl)
macro_rules! Depcrate_statsimpl_15 {
() => {
// Module: crate::stats
// Provides: {"impl_15"}
// Dependencies: {}
impl Default for StreamStats { fn default () -> Self { Self { duration_hist : Histogram :: < u64 > :: new (3) . unwrap () , throughput_hist : Histogram :: < u64 > :: new (3) . unwrap () , } } }
};
}
