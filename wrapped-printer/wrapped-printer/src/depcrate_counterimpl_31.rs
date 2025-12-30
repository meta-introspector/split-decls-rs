// Generated macro for impl_31 (impl)
macro_rules! Depcrate_counterimpl_31 {
() => {
// Module: crate::counter
// Provides: {"impl_31"}
// Dependencies: {}
impl < W : Write > CounterWriter < W > { pub (crate) fn new (wtr : W) -> CounterWriter < W > { CounterWriter { wtr , count : 0 , total_count : 0 } } }
};
}
