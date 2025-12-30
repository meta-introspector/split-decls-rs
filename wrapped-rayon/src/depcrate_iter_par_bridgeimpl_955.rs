// Generated macro for impl_955 (impl)
macro_rules! Depcrate_iter_par_bridgeimpl_955 {
() => {
// Module: crate::iter::par_bridge
// Provides: {"impl_955"}
// Dependencies: {}
impl < T > ParallelBridge for T where T : Iterator < Item : Send > + Send , { fn par_bridge (self) -> IterBridge < Self > { IterBridge { iter : self } } }
};
}
