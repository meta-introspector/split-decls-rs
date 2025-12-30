// Generated macro for impl_87 (impl)
macro_rules! Depcrate_future_future_flattenimpl_87 {
() => {
// Module: crate::future::future::flatten
// Provides: {"impl_87"}
// Dependencies: {}
impl < Fut > FusedFuture for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Future , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
};
}
