// Generated macro for impl_157 (impl)
macro_rules! Depcrate_bridge_serverimpl_157 {
() => {
// Module: crate::bridge::server
// Provides: {"impl_157"}
// Dependencies: {}
impl RunningSameThreadGuard { fn new () -> Self { let already_running = ALREADY_RUNNING_SAME_THREAD . replace (true) ; assert ! (! already_running , "same-thread nesting (\"reentrance\") of proc macro executions is not supported") ; RunningSameThreadGuard (()) } }
};
}
