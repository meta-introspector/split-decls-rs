// Generated macro for impl_122 (impl)
macro_rules! Depcrate_connectionimpl_122 {
() => {
// Module: crate::connection
// Provides: {"impl_122"}
// Dependencies: {}
impl From < & InternalBenchmarkId > for RawBenchmarkId { fn from (other : & InternalBenchmarkId) -> RawBenchmarkId { RawBenchmarkId { group_id : other . group_id . clone () , function_id : other . function_id . clone () , value_str : other . value_str . clone () , throughput : other . throughput . iter () . cloned () . collect () , } } }
};
}
