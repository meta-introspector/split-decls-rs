// Generated macro for impl_179 (impl)
macro_rules! Depcrate_future_future_sharedimpl_179 {
() => {
// Module: crate::future::future::shared
// Provides: {"impl_179"}
// Dependencies: {}
impl < Fut > Inner < Fut > where Fut : Future , { # [doc = " Safety: callers must first ensure that `self.inner.state`"] # [doc = " is `COMPLETE`"] unsafe fn output (& self) -> & Fut :: Output { match unsafe { & * self . future_or_output . get () } { FutureOrOutput :: Output (item) => item , FutureOrOutput :: Future (_) => unreachable ! () , } } }
};
}
