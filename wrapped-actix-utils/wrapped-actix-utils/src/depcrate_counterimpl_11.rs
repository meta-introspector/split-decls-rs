// Generated macro for impl_11 (impl)
macro_rules! Depcrate_counterimpl_11 {
() => {
// Module: crate::counter
// Provides: {"impl_11"}
// Dependencies: {}
impl CounterGuard { fn new (inner : Rc < CounterInner >) -> Self { inner . inc () ; CounterGuard (inner) } }
};
}
