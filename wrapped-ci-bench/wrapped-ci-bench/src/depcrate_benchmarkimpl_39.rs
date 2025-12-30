// Generated macro for impl_39 (impl)
macro_rules! Depcrate_benchmarkimpl_39 {
() => {
// Module: crate::benchmark
// Provides: {"impl_39"}
// Dependencies: {}
impl BenchmarkKind { # [doc = " Returns the [`ResumptionKind`] used in the handshake part of the benchmark"] pub fn resumption_kind (self) -> ResumptionKind { match self { Self :: Handshake (kind) => kind , Self :: Transfer => ResumptionKind :: No , } } }
};
}
