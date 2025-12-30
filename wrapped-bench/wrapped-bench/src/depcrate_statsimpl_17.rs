// Generated macro for impl_17 (impl)
macro_rules! Depcrate_statsimpl_17 {
() => {
// Module: crate::stats
// Provides: {"impl_17"}
// Dependencies: {}
impl TransferResult { pub fn new (duration : Duration , size : u64) -> Self { let throughput = throughput_bps (duration , size) ; TransferResult { duration , size , throughput , } } }
};
}
