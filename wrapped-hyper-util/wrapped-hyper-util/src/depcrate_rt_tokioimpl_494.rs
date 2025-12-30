// Generated macro for impl_494 (impl)
macro_rules! Depcrate_rt_tokioimpl_494 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_494"}
// Dependencies: {}
impl < Fut > Executor < Fut > for TokioExecutor where Fut : Future + Send + 'static , Fut :: Output : Send + 'static , { fn execute (& self , fut : Fut) { # [cfg (feature = "tracing")] tokio :: spawn (fut . in_current_span ()) ; # [cfg (not (feature = "tracing"))] tokio :: spawn (fut) ; } }
};
}
