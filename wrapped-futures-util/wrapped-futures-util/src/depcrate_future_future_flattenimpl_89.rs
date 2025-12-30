// Generated macro for impl_89 (impl)
macro_rules! Depcrate_future_future_flattenimpl_89 {
() => {
// Module: crate::future::future::flatten
// Provides: {"impl_89"}
// Dependencies: {}
impl < Fut > FusedStream for Flatten < Fut , Fut :: Output > where Fut : Future , Fut :: Output : Stream , { fn is_terminated (& self) -> bool { matches ! (self , Self :: Empty) } }
};
}
