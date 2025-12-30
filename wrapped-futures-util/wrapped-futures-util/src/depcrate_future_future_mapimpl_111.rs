// Generated macro for impl_111 (impl)
macro_rules! Depcrate_future_future_mapimpl_111 {
() => {
// Module: crate::future::future::map
// Provides: {"impl_111"}
// Dependencies: {}
impl < Fut , F , T > FusedFuture for Map < Fut , F > where Fut : Future , F : FnOnce1 < Fut :: Output , Output = T > , { fn is_terminated (& self) -> bool { match self { Self :: Incomplete { .. } => false , Self :: Complete => true , } } }
};
}
