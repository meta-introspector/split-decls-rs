// Generated macro for impl_132 (impl)
macro_rules! Depcrate_future_future_catch_unwindimpl_132 {
() => {
// Module: crate::future::future::catch_unwind
// Provides: {"impl_132"}
// Dependencies: {}
impl < Fut > CatchUnwind < Fut > where Fut : Future + UnwindSafe , { pub (super) fn new (future : Fut) -> Self { Self { future } } }
};
}
